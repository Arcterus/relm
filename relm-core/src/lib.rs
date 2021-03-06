/*
 * Copyright (c) 2017 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

extern crate futures;
extern crate glib_itc;
extern crate gtk;
extern crate tokio_core;

use std::collections::VecDeque;
use std::io::Error;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;

use futures::{Async, Poll, Stream};
use futures::task::{self, Task};
use glib_itc::Sender;
use tokio_core::reactor;
pub use tokio_core::reactor::{Handle, Remote};

pub struct Core { }

impl Core {
    pub fn run() -> Remote {
        let (sender, receiver) = channel();
        thread::spawn(move || {
            let mut core = reactor::Core::new().unwrap();
            sender.send(core.remote()).unwrap();
            loop {
                core.turn(None);
            }
        });

        receiver.recv().unwrap()
    }
}

struct _EventStream<MSG> {
    events: VecDeque<MSG>,
    observers: Vec<Box<Fn(MSG) + Send>>,
    sender: Arc<Mutex<Sender>>,
    task: Option<Task>,
    terminated: bool,
    ui_events: VecDeque<MSG>,
}

#[derive(Clone)]
pub struct EventStream<MSG> {
    stream: Arc<Mutex<_EventStream<MSG>>>,
}

impl<MSG> EventStream<MSG> {
    pub fn new(sender: Arc<Mutex<Sender>>) -> Self {
        EventStream {
            stream: Arc::new(Mutex::new(_EventStream {
                events: VecDeque::new(),
                observers: vec![],
                sender: sender,
                task: None,
                terminated: false,
                ui_events: VecDeque::new(),
            })),
        }
    }

    pub fn close(&self) -> Result<(), Error> {
        let mut stream = self.stream.lock().unwrap();
        stream.sender.lock().unwrap().close()?;
        stream.terminated = true;
        if let Some(ref task) = stream.task {
            task.unpark();
        }
        Ok(())
    }

    pub fn emit(&self, event: MSG)
        where MSG: Clone + 'static
    {
        let mut stream = self.stream.lock().unwrap();
        if let Some(ref task) = stream.task {
            task.unpark();
        }
        // TODO: try to avoid clone by sending a reference.
        stream.events.push_back(event.clone());

        for observer in &stream.observers {
            observer(event.clone());
        }
    }

    fn get_event(&self) -> Option<MSG> {
        self.stream.lock().unwrap().events.pop_front()
    }

    fn is_terminated(&self) -> bool {
        let stream = self.stream.lock().unwrap();
        stream.terminated
    }

    pub fn observe<CALLBACK: Fn(MSG) + Send + 'static>(&self, callback: CALLBACK) {
        self.stream.lock().unwrap().observers.push(Box::new(callback));
    }

    pub fn pop_ui_events(&self) -> Option<MSG> {
        self.stream.lock().unwrap().ui_events.pop_front()
    }
}

impl<MSG: Clone + 'static> Stream for EventStream<MSG> {
    type Item = MSG;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if self.is_terminated() {
            Ok(Async::Ready(None))
        }
        else {
            match self.get_event() {
                Some(event) => {
                    let mut stream = self.stream.lock().unwrap();
                    stream.task = None;
                    stream.ui_events.push_back(event.clone());
                    stream.sender.lock().unwrap().send();
                    // TODO: try to avoid clone by sending a reference.
                    Ok(Async::Ready(Some(event)))
                },
                None => {
                    let mut stream = self.stream.lock().unwrap();
                    stream.task = Some(task::park());
                    Ok(Async::NotReady)
                },
            }
        }
    }
}
