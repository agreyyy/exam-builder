use dioxus::prelude::*;
#[inline_props]
pub fn Exam(cx: Scope, name: String) -> Element {
    render!(
        div {
            "exam: {name}"
        }
    )
}