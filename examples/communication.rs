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

use gtk::{
    Button,
    ButtonExt,
    ContainerExt,
    EditableSignals,
    Entry,
    EntryExt,
    Inhibit,
    Label,
    WidgetExt,
    Window,
    WindowType,
};
use gtk::Orientation::{Horizontal, Vertical};
use relm::{Component, ContainerWidget, Relm, Widget};

use self::CounterMsg::*;
use self::Msg::*;
use self::TextMsg::*;

#[derive(Clone)]
struct TextModel {
    content: String,
}

#[derive(Msg)]
enum TextMsg {
    Change(String),
}

#[derive(Clone)]
struct Text {
    label: Label,
    vbox: gtk::Box,
}

impl Widget for Text {
    type Model = TextModel;
    type ModelParam = ();
    type Msg = TextMsg;
    type Root = gtk::Box;

    fn model(_: ()) -> TextModel {
        TextModel {
            content: String::new(),
        }
    }

    fn root(&self) -> &Self::Root {
        &self.vbox
    }

    fn update(&mut self, event: TextMsg, model: &mut TextModel) {
        match event {
            Change(text) => {
                model.content = text.chars().rev().collect();
                self.label.set_text(&model.content);
            },
        }
    }

    fn view(relm: &Relm<Self>, _model: &TextModel) -> Self {
        let vbox = gtk::Box::new(Vertical, 0);

        let input = Entry::new();
        vbox.add(&input);

        let label = Label::new(None);
        vbox.add(&label);

        let input2 = input.clone();
        connect!(relm, input, connect_changed(_), Change(input2.get_text().unwrap()));

        Text {
            label: label,
            vbox: vbox,
        }
    }
}

#[derive(Clone)]
struct CounterModel {
    counter: i32,
}

#[derive(Msg)]
enum CounterMsg {
    Decrement,
    Increment,
}

#[derive(Clone)]
struct Counter {
    counter_label: Label,
    vbox: gtk::Box,
}

impl Widget for Counter {
    type Model = CounterModel;
    type ModelParam = ();
    type Msg = CounterMsg;
    type Root = gtk::Box;

    fn model(_: ()) -> CounterModel {
        CounterModel {
            counter: 0,
        }
    }

    fn root(&self) -> &Self::Root {
        &self.vbox
    }

    fn update(&mut self, event: CounterMsg, model: &mut CounterModel) {
        let label = &self.counter_label;

        match event {
            Decrement => {
                model.counter -= 1;
                label.set_text(&model.counter.to_string());
            },
            Increment => {
                model.counter += 1;
                label.set_text(&model.counter.to_string());
            },
        }
    }

    fn view(relm: &Relm<Self>, _model: &CounterModel) -> Self {
        let vbox = gtk::Box::new(Vertical, 0);

        let plus_button = Button::new_with_label("+");
        vbox.add(&plus_button);

        let counter_label = Label::new("0");
        vbox.add(&counter_label);

        let minus_button = Button::new_with_label("-");
        vbox.add(&minus_button);

        connect!(relm, plus_button, connect_clicked(_), Increment);
        connect!(relm, minus_button, connect_clicked(_), Decrement);

        Counter {
            counter_label: counter_label,
            vbox: vbox,
        }
    }
}

#[derive(Clone)]
struct Model {
    counter: i32,
}

#[derive(Msg)]
enum Msg {
    TextChange(String),
    Quit,
}

#[derive(Clone)]
struct Win {
    _counter1: Component<Counter>,
    _counter2: Component<Counter>,
    label: Label,
    _text: Component<Text>,
    window: Window,
}

impl Widget for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;
    type Root = Window;

    fn model(_: ()) -> Model {
        Model {
            counter: 0,
        }
    }

    fn root(&self) -> &Self::Root {
        &self.window
    }

    fn update(&mut self, event: Msg, model: &mut Model) {
        match event {
            TextChange(text) => {
                println!("{}", text);
                model.counter += 1;
                self.label.set_text(&model.counter.to_string());
            },
            Quit => gtk::main_quit(),
        }
    }

    fn view(relm: &Relm<Self>, _model: &Model) -> Self {
        let window = Window::new(WindowType::Toplevel);

        let hbox = gtk::Box::new(Horizontal, 0);

        let button = Button::new_with_label("Decrement");
        hbox.add(&button);

        let label = Label::new(None);

        let counter1 = hbox.add_widget::<Counter, _>(&relm, ());
        let counter2 = hbox.add_widget::<Counter, _>(&relm, ());
        let text = hbox.add_widget::<Text, _>(&relm, ());
        hbox.add(&label);

        let win = Win {
            _counter1: counter1.clone(),
            _counter2: counter2.clone(),
            label: label.clone(),
            _text: text.clone(),
            window: window.clone(),
        };

        connect!(text@Change(text), relm, TextChange(text));
        connect!(text@Change(_), counter1, win.inc());
        connect!(counter1@Increment, counter2, Decrement);
        connect!(button, connect_clicked(_), counter1, Decrement);

        window.add(&hbox);

        window.show_all();

        connect!(relm, window, connect_delete_event(_, _) (Some(Quit), Inhibit(false)));

        Win {
            _counter1: counter1,
            _counter2: counter2,
            label: label,
            _text: text,
            window: window,
        }
    }
}

impl Win {
    fn inc(&self) -> CounterMsg {
        Increment
    }
}

fn main() {
    Win::run(()).unwrap();
}
