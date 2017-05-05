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

extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use std::cell::RefCell;
use std::rc::Rc;

use gtk::{
    ButtonExt,
    CheckButton,
    ContainerExt,
    Inhibit,
    WidgetExt,
    Window,
    WindowType,
};
use gtk::Orientation::Vertical;
use relm::{Relm, RemoteRelm, Widget};

use self::Msg::*;

#[derive(Clone)]
struct Model {
    check: bool,
}

#[derive(Msg)]
enum Msg {
    Check,
    Quit,
}

#[derive(Clone)]
struct Win {
    //model: Model,
    window: Window,
}

impl Widget for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;
    type Root = Window;

    fn model(_: ()) -> Model {
        Model {
            check: false,
        }
    }

    fn root(&self) -> &Self::Root {
        &self.window
    }

    fn update(&mut self, event: Msg, model: &mut Model) {
        match event {
            Check => model.check = true,
            Quit => gtk::main_quit(),
        }
    }

    fn view(relm: &RemoteRelm<Self>, model: &Self::Model) -> Self {
        let vbox = gtk::Box::new(Vertical, 0);

        let plus_button = CheckButton::new_with_label("+");
        vbox.add(&plus_button);

        let window = Window::new(WindowType::Toplevel);
        window.add(&vbox);
        window.show_all();

        connect!(relm, plus_button, connect_clicked(_), Check);
        connect!(relm, window, connect_delete_event(_, _) (Some(Quit), Inhibit(false)));
        connect!(relm@Check, relm, Quit);

        Win {
            //model,
            window: window,
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
