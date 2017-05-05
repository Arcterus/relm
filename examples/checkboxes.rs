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
    ContainerExt,
    Inhibit,
    ToggleButtonExt,
    WidgetExt,
    Window,
    WindowType,
};
use gtk::Orientation::Vertical;
use relm::{Component, ContainerWidget, Relm, Widget};

use self::CheckMsg::*;
use self::Msg::*;

struct CheckModel {
    check: bool,
    label: &'static str,
}

#[derive(Msg)]
enum CheckMsg {
    Check,
    Toggle,
    Uncheck,
}

struct CheckButton {
    button: gtk::CheckButton,
    model: CheckModel,
    relm: Relm<CheckButton>,
}

impl Widget for CheckButton {
    type Model = CheckModel;
    type ModelParam = &'static str;
    type Msg = CheckMsg;
    type Root = gtk::CheckButton;

    fn model(label: &'static str) -> CheckModel {
        CheckModel {
            check: false,
            label,
        }
    }

    fn root(&self) -> Self::Root {
        self.button.clone()
    }

    fn update(&mut self, event: CheckMsg) {
        match event {
            Check => {
                self.model.check = true;
                self.relm.stream().lock();
                self.button.set_active(true);
                self.relm.stream().unlock();
            },
            Toggle => {
                self.model.check = !self.model.check;
                self.button.set_active(self.model.check);
            },
            Uncheck => {
                self.model.check = false;
                self.relm.stream().lock();
                self.button.set_active(false);
                self.relm.stream().unlock();
            },
        }
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Rc<RefCell<Self>> {
        let button = gtk::CheckButton::new_with_label(model.label);

        connect!(relm, button, connect_clicked(_), Toggle);

        Rc::new(RefCell::new(CheckButton {
            button,
            model,
            relm: relm.clone(),
        }))
    }
}

#[derive(Msg)]
enum Msg {
    MinusWrapper(CheckMsg),
    Quit,
    PlusWrapper(CheckMsg),
}

struct Win {
    minus_button: Component<CheckButton>,
    plus_button: Component<CheckButton>,
    window: Window,
}

impl Widget for Win {
    type Model = ();
    type ModelParam = ();
    type Msg = Msg;
    type Root = Window;

    fn model(_: ()) -> () {
    }

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn update(&mut self, event: Msg) {
        match event {
            Quit => gtk::main_quit(),
            MinusWrapper(Toggle) => {
                if self.minus_button.widget().model.check {
                    self.plus_button.stream().emit(Uncheck);
                }
                else {
                    self.plus_button.stream().emit(Check);
                }
            },
            PlusWrapper(Toggle) => {
                if self.plus_button.widget().model.check {
                    self.minus_button.stream().emit(Uncheck);
                }
                else {
                    self.minus_button.stream().emit(Check);
                }
            },
            MinusWrapper(Check) | MinusWrapper(Uncheck) | PlusWrapper(Check) | PlusWrapper(Uncheck) => (),
        }
    }

    fn view(relm: &Relm<Self>, _model: Self::Model) -> Rc<RefCell<Self>> {
        let vbox = gtk::Box::new(Vertical, 0);

        let plus_button = vbox.add_widget::<CheckButton, _>(relm, "+");
        let minus_button = vbox.add_widget::<CheckButton, _>(relm, "-");

        let window = Window::new(WindowType::Toplevel);
        window.add(&vbox);
        window.show_all();

        //connect!(relm, plus_button, connect_clicked(_), Check);
        connect!(relm, window, connect_delete_event(_, _) (Some(Quit), Inhibit(false)));
        connect!(plus_button, relm, PlusWrapper);
        connect!(minus_button, relm, MinusWrapper);

        Rc::new(RefCell::new(Win {
            minus_button,
            plus_button,
            window: window,
        }))
    }
}

fn main() {
    Win::run(()).unwrap();
}
