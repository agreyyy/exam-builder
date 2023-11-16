#![allow(non_snake_case)]

mod exam_component;
mod exam_formats;

use dioxus_router::prelude::*;
use dioxus::prelude::*;
use log::LevelFilter;
use exam_component::Exam;
use exam_formats::QuestionFormat;


fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    log::info!("starting app");
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/exam/:name")]
    Exam { name: String },
}

#[inline_props]
fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            NewExam {}
            ExistingExams{ name: "agrey".into() }
        }
    })
}

fn NewExam(cx: Scope) -> Element {
    let exam_creation_chosen = use_state(cx, || {false});
    let new_exam_name = use_state(cx, || {String::new()});

    let css_state = if new_exam_name.is_empty() {
        "none"
    } else {
        "auto"
    };

    render!(
        div {
            button {
                onclick: move |_| {
                    exam_creation_chosen.set(true);
                },
                "create new exam"
            }
            if *exam_creation_chosen.get() {
                rsx!(
                    input {
                        placeholder: "exam name",
                        oninput: move |ev| {
                            new_exam_name.set(ev.data.value.clone());
                        }
                    },
                    
                    button {
                        style: "pointer-events:{css_state};",
                        Link {
                            to: "/exam/{new_exam_name}",
                            "✔️"
                        }
                    }
                )
            }
        }
    )
}

#[inline_props]
fn ExistingExams(cx: Scope, name: String) -> Element {
    let exams = use_state(cx, || None);

    use_effect(cx, name, |name| {
        to_owned![name, exams];
        async move {
            exams.set(server_lookup(name));
        }
    });

    render!(
        match exams.get() {
            Some(exams) => {
                rsx!(
                    ol {
                        for exam in exams.iter() {
                            rsx!(
                                li {
                                    Link{to:Route::Exam{ name: exam.to_string() }, "{exam}"}
                                }
                            )
                        }
                    }
                )
            }
            None => {
                rsx!(p{"You have no saved exams"})
            }
        }
    )
}

fn server_lookup(_name: String) -> Option<Vec<String>> {
    Some(vec!["AASLP1".into(), "AAHLP1".into()])
}
