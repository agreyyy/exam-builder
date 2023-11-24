use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct MediaProps {
    media: UseState<Option<String>>
}

pub fn QuestionMeda(cx:Scope<MediaProps>) -> Element {
    render!("IMG")
}