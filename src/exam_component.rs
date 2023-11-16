use dioxus::prelude::*;  
use crate::QuestionFormat;

#[inline_props]
pub fn Exam(cx: Scope, name: String) -> Element {
    let questions = use_ref(cx, || {Vec::new()});

    render!(
        div {
            "exam: {name}"
            ol {
                questions.read().iter().enumerate().map(|(i,question)| {
                    rsx!( 
                        li {
                            QuestionUI { 
                                question_data: questions.clone(),
                                question: question.clone(),
                                parent_idx: i,
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
           
        },

        for q in questions.read().iter() {
            rsx!(
                "{q:#?}"
            )
        }
    )
}

fn QuestionUI(cx: Scope<QuestionUIProps>) -> Element {
    
    let title = use_state(cx,|| {(&cx.props.question.title).to_owned()});
    let media = use_state(cx, || {(&cx.props.question.media).to_owned()});
    let format = use_ref(cx, || {(&cx.props.question.format).to_owned()});
    let sub_questions = use_ref(cx,|| {(&cx.props.question.sub_questions).to_owned()});
    let question_data = &cx.props.question_data;
    let parents_id = &cx.props.parent_idx;

    use_effect(cx, (title,media,format,sub_questions), |(title,media,format,sub_questions)| {
        to_owned![title,media,format,sub_questions, question_data, parents_id];
        async move {
            question_data.with_mut(|questions| {
                let title = title.get().clone();
                let media = media.get().clone();
                let format = format.read().to_owned();
                let sub_questions = sub_questions.read().to_owned();

                let new_question = Question::new(title, media, format, sub_questions);
                questions[parents_id] = new_question;
            })
        }
    });

    render!(
        div {
            input { 
                placeholder: "Enter the Question",
                value: "{title}",
                oninput: move |ev| {
                    title.set(ev.data.value.clone());
                }
            }

            QuestionFormat { format: format.clone() }
        }
        ol {
            sub_questions.read().iter().enumerate().map(|(i,question)| {
                rsx!(
                    li {
                        "{parents_id}"
                        QuestionUI { 
                            question_data: sub_questions.clone(),
                            question: question.clone(),
                            parent_idx: i,
                        }
                    }
                )
            })

            "{question_data.read():?}"
        }

        button {
            onclick: move |_ev| {
                sub_questions.with_mut(|questions| {
                    questions.push(Question::default());
                });
            },
            "+ add sub question"
        } 
        
        button {
            onclick: move |_ev| {
                question_data.with_mut(|questions| {
                    questions.remove(*parents_id);
                });
            },
            "- remove question"
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

#[derive(Props, PartialEq)]
struct QuestionUIProps{
    question_data: UseRef<Vec<Question>>,
    question: Question,
    parent_idx: usize,
}

impl Default for Question {
    fn default() -> Self {
        use QuestionFormat::*;
        Question { title: "New Question".into(), media: None, format: ShortAnswer, sub_questions: Vec::new() }
    }
}

impl Question {
    fn new(title: String,media: Option<String>,format: QuestionFormat,sub_questions: Vec<Question>,) -> Self {
        Question { title, media, format, sub_questions }
    }
}
