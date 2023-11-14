use std::fs::File;

use dioxus::prelude::*;  
#[inline_props]
pub fn Exam(cx: Scope, name: String) -> Element {
    let questions = use_ref(cx, || {Vec::new()});

    render!(
        div {
            "exam: {name}"
            ol {
                questions.read().iter().map(|question: &Question| {
                    rsx!( 
                        li {
                            QuestionUI { 
                                title: question.title.clone(),
                                media: question.media.clone(),
                                format: question.format.clone(),
                                sub_questions: question.sub_questions.clone()
                            }
                        }
                    )
                })
            }
            button {
                onclick: move |_ev| {
                  questions.with_mut(|questions| {
                    questions.push(Question::default());
                  })  
                },
                "+ add question"
            }
        }
    )
}

fn QuestionUI(cx: Scope<Question>) -> Element {
    
    let title = use_state(cx,|| {(&cx.props.title).to_owned()});
    let sub_questions = use_ref(cx,|| {(&cx.props.sub_questions).to_owned()});

    render!(
        input { 
            placeholder: "Enter the Question",
            value: "{title}",
            oninput: move |ev| {
                title.set(ev.data.value.clone());
            }
        }
        ol {
            sub_questions.read().iter().map(|question: &Question| {
                rsx!(
                    li {
                        QuestionUI { 
                            title: question.title.clone(),
                            media: question.media.clone(),
                            format: question.format.clone(),
                            sub_questions: question.sub_questions.clone()
                        }
                    }
                )
            })
        }

        button {
            onclick: move |_ev| {
                sub_questions.with_mut(|questions| {
                    questions.push(Question::default());
                })
            },
            "+ add sub question"
        }
    )
}

#[derive(Props, PartialEq, Debug,Clone)]
struct Question {
    title: String,
    #[props(!optional)]
    media: Option<String>,
    format: QuestionFormat,
    sub_questions: Vec<Question>,
}

impl Default for Question {
    fn default() -> Self {
        use QuestionFormat::*;
        Question { title: "New Question".into(), media: None, format: ShortAnswer, sub_questions: Vec::new() }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum QuestionFormat {
    ShortAnswer,
    MultipleChoice(Vec<String>),
    Justify,
    TrueOrFalse(u8, Vec<String>)  
}