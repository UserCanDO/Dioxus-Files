use dioxus::{prelude::*, html::style};

#[derive(PartialEq, Props)]
pub struct ContextProp{ 
    #[props(default = false)]
    visible: bool,
    top: i32, left:  i32
}
pub fn FolderContext(cx: Scope<ContextProp>) -> Element {
    let visible = if cx.props.visible { "context-visible" } else { "" };
    cx.render(rsx! {
        style { include_str!("../css/context.css") }
        div {
            class: "context-menu {visible}", style: "top: {cx.props.top}px; left: {cx.props.left}px;",
            div{ class: "item", 
                img{ src: "src/icons/folder.png" }, "add file" 
            },
            div{ class: "item", "add folder" },
            div{ class: "item", "rename" },
            div{ class: "item", "move" },
            div{ class: "item", "copy" },
            div{ class: "item", "paste" },
            div{ class: "item", "delete" },
        }
    })
}

pub fn FileContext(cx: Scope<ContextProp>) -> Element {
    let visible = if cx.props.visible { "context-visible" } else { "" };
    cx.render(rsx! {
        style { include_str!("../css/context.css") }
        div {
            class: "context-menu {visible}", style: "top: {cx.props.top}px; left: {cx.props.left}px;",
            div{ class: "item", "edit" },
            div{ class: "item", "rename" },
            div{ class: "item", "move" },
            div{ class: "item", "copy" },
            div{ class: "item", "paste" },
            hr{}
            div{ class: "item", "delete" },
        }
    })
}


/*<div id="context-menu">
      <div class="item">Option 1</div>
      <div class="item">Option 2</div>
      <div class="item">Option 3</div>
      <div class="item">Option 4</div>
      <div class="item">Option 5</div>
    </div> */