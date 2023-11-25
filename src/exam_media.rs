use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct MediaProps {
    media: UseState<Option<String>>
}

pub fn QuestionMedia(cx:Scope<MediaProps>) -> Element {
    render!("IMG")
}