#![allow(non_snake_case)]

mod exam_component;

use dioxus_router::prelude::*;
use dioxus::prelude::*;
use log::LevelFilter;
use exam_component::Exam;

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
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/exam/:name")]
    Exam { name: String },
}

#[inline_props]
fn Blog(cx: Scope, id: i32) -> Element {
    render! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[inline_props]
fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            Link{ to: Route::Exam{ name: String::new() }, "Create new exam"}
            ExistingExams{ name: "agrey".into() }
        }
    })
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
