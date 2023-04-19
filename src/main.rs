#![allow(non_snake_case)]

use dioxus::prelude::*;

mod components;
use crate::components::Tree;

mod items;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "My Website",
            Tree{}
        }
    })
}