use dioxus::prelude::*;
use crate::{items::TreeItem, components::{FolderContext, FileContext}};

fn test_tree()-> TreeItem{ 
  return TreeItem::Folder("Root".to_string(), vec![
      TreeItem::Folder("Folder 1".to_string(), vec![
          TreeItem::File("File 1".to_string()),
          TreeItem::File("File 2".to_string()),
      ]),
      TreeItem::Folder("Folder 2".to_string(), vec![
          TreeItem::File("File 3".to_string()),
          TreeItem::File("File 4".to_string()),
          TreeItem::Folder("Folder 3".to_string(), vec![
            TreeItem::File("File 5".to_string()),
            TreeItem::File("File 6".to_string()),
            TreeItem::File("File 7".to_string()),
        ]),
      ]),
  ]);
}

#[derive(PartialEq, Props)]
struct  TreeItemModelProp{ item: TreeItem }

fn TreeItemModel(cx: Scope<TreeItemModelProp>)->Element{
    match &cx.props.item {
        TreeItem::File(name) => {
            let context_visible = use_state(cx, || false);
            let context_position = use_state(cx,|| (0, 0));

            let hilight = if *context_visible.get(){ "text-orange"} else { "" };

            return cx.render(rsx!( 
                li{
                    class:"file {hilight}",
                    prevent_default: "oncontextmenu",
                    oncontextmenu: move |event| {
                        context_visible.set(false);

                        context_position.set(event.page_coordinates().to_i32().to_tuple());
                        context_visible.set(true);
                        
                    },
                    "{name}",
                    FileContext{ visible: **context_visible, top: context_position.1, left: context_position.0, onclick: move |__event| {
                      context_visible.set(false);
                    } } 
                }
            )) 
        },
        TreeItem::Folder( name, children) =>{
            let is_nested = use_state(cx, || true);

            let context_visible = use_state(cx, || false);
            let context_position = use_state(cx,|| (0, 0));

            let nested = if *is_nested.get() { "nested" } else { "nested active" };
            let caret = if *is_nested.get() { "caret" } else { "caret caret-down" };
            let hilight = if *context_visible.get(){ "text-orange"} else { "" };
            
            return cx.render(rsx!(
                li{span{ 
                    class: "{caret} {hilight}",
                    prevent_default: "oncontextmenu",
                    onclick: move |__event| is_nested.set(!is_nested.get()),
                    oncontextmenu: move |event| {
                        context_visible.set(false);

                        context_position.set(event.page_coordinates().to_i32().to_tuple());
                        context_visible.set(true);
                        
                    },
                    "{name}" },
                    FolderContext{ visible: **context_visible, top: context_position.1, left: context_position.0, onclick: move |__event| {
                      context_visible.set(false);
                    } },
                ul{
                    class: "{nested}",
                    children.iter().map(|child| rsx!{
                        TreeItemModel { item: child.clone() }
                    }),
                }}
            ));
        }
    }
}

pub fn Tree(cx: Scope) -> Element {
    let tree: TreeItem = test_tree();

    return cx.render(rsx! {
        style { include_str!("../css/tree.css") }
        ul {
          id: "myUl",
          TreeItemModel{ item: tree }
        }
    });
}

/* 
<ul id="myUL">
  <li><span class="caret">Beverages</span>
    <ul class="nested">
      <li>Water</li>
      <li>Coffee</li>
      <li><span class="caret">Tea</span>
        <ul class="nested">
          <li>Black Tea</li>
          <li>White Tea</li>
          <li><span class="caret">Green Tea</span>
            <ul class="nested">
              <li>Sencha</li>
              <li>Gyokuro</li>
              <li>Matcha</li>
              <li>Pi Lo Chun</li>
            </ul>
          </li>
        </ul>
      </li>
    </ul>
  </li>
</ul>  */