use dioxus::prelude::*;

#[derive(Props)]
pub struct ContextProp<'a>{ 
    #[props(default = false)]
    visible: bool,
    top: i32, left:  i32,
    onclick: EventHandler<'a, MouseEvent>,

    children: Element<'a>,
}
pub fn FolderContext<'a>(cx: Scope<'a, ContextProp<'a>>) -> Element<'a> {
    let visible = if cx.props.visible { "context-visible" } else { "" };
    cx.render(rsx! {
        style { include_str!("../css/context.css") }
        div {
            class: "context-menu {visible}", style: "top: {cx.props.top}px; left: {cx.props.left}px;",
            div{ class: "item", onclick: move|event|{ cx.props.onclick.call(event); }, "\u{1F5B9} add file" },
            div{ class: "item", onclick: move|event|{ cx.props.onclick.call(event); }, "\u{1F5C1} add folder" },
            div{ class: "item", onclick: move|event|{ cx.props.onclick.call(event); },"\u{1F589} rename" },
            div{ class: "item", onclick: move|event|{ cx.props.onclick.call(event); }, "\u{2704} move" },
            div{ class: "item", onclick: move|event|{ cx.props.onclick.call(event); }, "\u{1F5D0} copy" },
            div{ class: "item", onclick: move|event|{ cx.props.onclick.call(event); }, "\u{1F5CD} paste" },
            div{ class: "item", onclick: move|event|{ cx.props.onclick.call(event); }, "\u{1F5D1} delete" },
        }
    })
}

pub fn FileContext<'a>(cx: Scope<'a, ContextProp<'a>>) -> Element<'a> {
    let visible = if cx.props.visible { "context-visible" } else { "" };
    cx.render(rsx! {
        style { include_str!("../css/context.css") }
        div {
            class: "context-menu {visible}", style: "top: {cx.props.top}px; left: {cx.props.left}px;",
            div{ class: "item", onclick: move|event|{ cx.props.onclick.call(event); }, "\u{1F58E} edit" },
            div{ class: "item", onclick: move|event|{ cx.props.onclick.call(event); }, "\u{1f589} rename" },
            div{ class: "item", onclick: move|event|{ cx.props.onclick.call(event); }, "\u{2704} move" },
            div{ class: "item", onclick: move|event|{ cx.props.onclick.call(event); }, "\u{1F5D0} copy" },
            div{ class: "item", onclick: move|event|{ cx.props.onclick.call(event); }, "\u{1F5CD} paste" },
            div{ class: "item", onclick: move|event|{ cx.props.onclick.call(event); }, "\u{1F5D1} delete" },
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