use dioxus::prelude::*;

pub fn NavBar(cx: Scope) -> Element {
    return cx.render(rsx! {
        style { include_str!("../css/navbar.css") }
        div {
            class: "nav bg-orange",
            div{ class: "nav-brand", "Dioxus File"},
            a{ class: "nav-item", "Home" }
        }
    });
}

/*<div  class='nav'>
    <div class='nav-brand'>Axum Alpine</div>
    <a class='nav-item $home' href="$url">Home</a>
    <a class='nav-item $users' href="$url/users">Users</a>
    <a class='nav-item $about' href="$url/about">About</a>
</div> */