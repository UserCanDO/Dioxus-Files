use dioxus::prelude::*;

use crate::components::Tree;

pub fn SideBar(cx: Scope) -> Element {
    return cx.render(rsx! (
        style{ include_str!("../css/sidebar.css") },
        div { 
            class:"sidebar",
            Tree{}
        }
    ));
}