#![allow(non_snake_case)]

use dioxus::prelude::*;

mod components;
use crate::components::{SideBar, NavBar, Main};

mod items;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        style{ include_str!("css/style.css") }
        div {
            NavBar{},
            SideBar{},
            Main{}
        }
    })
}