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

#![feature(proc_macro)]

extern crate gtk;
#[macro_use]
extern crate relm;
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;

use gtk::{
    ContainerExt,
    Frame,
    Inhibit,
    Label,
    WidgetExt,
    Window,
};
use gtk::Orientation::{Horizontal, Vertical};
use gtk::WindowType::Toplevel;
use relm::{Cast, Component, Container, ContainerWidget, Relm, RelmContainer, Widget};

use self::Msg::*;

#[derive(Clone)]
struct CenterButton {
    button: gtk::Button,
}

impl Widget for CenterButton {
    type Model = ();
    type ModelParam = ();
    type Msg = ();
    type Root = gtk::Button;

    fn model(_: ()) -> () {
    }

    fn parent_id() -> Option<&'static str> {
        Some("center")
    }

    fn root(&self) -> &Self::Root {
        &self.button
    }

    fn update(&mut self, _msg: (), _model: &mut ()) {
    }

    fn view(_relm: &Relm<Self>, _model: &()) -> Self {
        let button = gtk::Button::new_with_label("-");
        CenterButton {
            button: button,
        }
    }
}

#[derive(Clone)]
struct Button {
    button: gtk::Button,
}

impl Widget for Button {
    type Model = ();
    type ModelParam = ();
    type Msg = ();
    type Root = gtk::Button;

    fn model(_: ()) -> () {
    }

    fn parent_id() -> Option<&'static str> {
        Some("right")
    }

    fn root(&self) -> &Self::Root {
        &self.button
    }

    fn update(&mut self, _msg: (), _model: &mut ()) {
    }

    fn view(_relm: &Relm<Self>, _model: &()) -> Self {
        let button = gtk::Button::new_with_label("+");
        Button {
            button: button,
        }
    }
}

#[derive(Clone)]
struct MyFrame {
    frame: Frame,
}

impl Widget for MyFrame {
    type Model = ();
    type ModelParam = ();
    type Msg = ();
    type Root = Frame;

    fn model(_: ()) -> () {
    }

    fn root(&self) -> &Self::Root {
        &self.frame
    }

    fn update(&mut self, _msg: (), _model: &mut ()) {
    }

    fn view(_relm: &Relm<Self>, _model: &()) -> Self {
        let frame = Frame::new(None);
        MyFrame {
            frame,
        }
    }
}

impl Container for MyFrame {
    type Container = Frame;

    fn container(&self) -> &Self::Container {
        &self.frame
    }
}

#[derive(Clone)]
struct SplitBox {
    hbox1: gtk::Box,
    hbox2: Frame,
    hbox3: Component<MyFrame>,
    vbox: gtk::Box,
}

impl Container for SplitBox {
    type Container = gtk::Box;

    fn container(&self) -> &Self::Container {
        &self.hbox1
    }

    fn add_widget<WIDGET: Widget>(&self, widget: &WIDGET) -> gtk::Container {
        if WIDGET::parent_id() == Some("right") {
            self.hbox3.add(widget.root());
            self.hbox3.widget().root().clone().upcast()
        }
        else if WIDGET::parent_id() == Some("center") {
            self.hbox2.add(widget.root());
            self.hbox2.clone().upcast()
        }
        else {
            self.hbox1.add(widget.root());
            self.hbox1.clone().upcast()
        }
    }
}

impl Widget for SplitBox {
    type Model = ();
    type ModelParam = ();
    type Msg = ();
    type Root = gtk::Box;

    fn model(_: ()) -> () {
        ()
    }

    fn root(&self) -> &Self::Root {
        &self.vbox
    }

    fn update(&mut self, _event: (), _model: &mut ()) {
    }

    fn view(relm: &Relm<Self>, _model: &Self::Model) -> Self {
        let vbox = gtk::Box::new(Horizontal, 0);
        let hbox1 = gtk::Box::new(Vertical, 0);
        vbox.add(&hbox1);
        let hbox2 = Frame::new(None);
        vbox.add(&hbox2);
        let hbox3 = vbox.add_widget::<MyFrame, _>(relm, ());
        SplitBox {
            hbox1,
            hbox2,
            hbox3,
            vbox,
        }
    }
}

#[derive(Msg)]
pub enum Msg {
    Quit,
}

#[derive(Clone)]
struct Win {
    button: Component<Button>,
    center_button: Component<CenterButton>,
    vbox: Component<SplitBox>,
    window: Window,
}

impl Widget for Win {
    type Model = ();
    type ModelParam = ();
    type Msg = Msg;
    type Root = Window;

    fn model(_: ()) -> () {
    }

    fn root(&self) -> &Self::Root {
        &self.window
    }

    fn update(&mut self, event: Msg, _model: &mut ()) {
        match event {
            Quit => gtk::main_quit(),
        }
    }

    fn view(relm: &Relm<Self>, _model: &()) -> Self {
        let window = Window::new(Toplevel);
        let vbox = window.add_widget::<SplitBox, _>(&relm, ());
        let plus_button = gtk::Button::new_with_label("+");
        vbox.add(&plus_button);
        let label = Label::new(Some("0"));
        vbox.add(&label);
        let button = vbox.add_widget::<Button, _>(&relm, ());
        let center_button = vbox.add_widget::<CenterButton, _>(&relm, ());
        let minus_button = gtk::Button::new_with_label("-");
        vbox.add(&minus_button);
        connect!(relm, window, connect_delete_event(_, _) (Some(Quit), Inhibit(false)));
        window.show_all();
        Win {
            button: button,
            center_button,
            vbox: vbox,
            window: window,
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
