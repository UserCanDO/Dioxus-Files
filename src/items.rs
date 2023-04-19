use dioxus::prelude::Props;

#[derive(Clone, PartialEq)]
pub enum TreeItem {
    Folder(String, Vec<TreeItem>),
    File(String),
}