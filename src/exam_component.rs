use dioxus::prelude::*; 
use crate::QuestionFormat;
use crate::QuestionMeda;
#[inline_props]
pub fn Exam(cx: Scope, name: String) -> Element {
    let questions = use_ref(cx, || {Vec::<Question>::new()});
    let mut counter = use_state(cx, || 0);

    render!(
        "{name}"
        ol {
            questions.read().iter().enumerate().map( move |(i,question)| {
                rsx!(
                    li {
                        key: "{question.id}",
                        QuestionUI { 
                            question_data: questions.clone(),
                            question: question.clone(),
                            parent_idx: i,
                        }
                        button {
                            onclick: move |_ev| {
                                questions.with_mut(|questions| {
                                    questions.remove(i);
                                });
                            },
                            "- remove question with id of {question.id.clone()}"
                        }
                    }
                )
            }
        )}

        button {
            onclick: move |_ev| {
                counter += 1;
                questions.with_mut(|questions| {
                    questions.push(Question::new_default(counter.get().to_owned()));
                })  
            },
            "+ add question" 
        }


        "counter = {counter}"
        // for q in questions.read().iter() {
        //     rsx!(
        //         "{next_id}"
        //         "{q:#?}"
        //     )
        // }
    )
}

fn QuestionUI(cx: Scope<QuestionUIProps>) -> Element {
    
    let title = use_state(cx,|| {(&cx.props.question.title).to_owned()});
    let media = use_state(cx, || {(&cx.props.question.media).to_owned()});
    let format = use_ref(cx, || {(&cx.props.question.format).to_owned()});
    let sub_questions = use_ref(cx,|| {(&cx.props.question.sub_questions).to_owned()});
    let question_data = &cx.props.question_data;
    let mut counter = use_state(cx, || 0_usize);
    let parents_id = &cx.props.parent_idx;
    let id = &cx.props.question.id;

    use_effect(cx, (title,media,format,sub_questions), |(title,media,format,sub_questions)| {
        to_owned![title,media,format,sub_questions, question_data, parents_id, id];
        async move {
            question_data.with_mut(|questions| {
                let title = title.get().clone();
                let media = media.get().clone();
                let format = format.read().to_owned();
                let sub_questions = sub_questions.read().to_owned();
                let new_question = Question::new(title, media, format, sub_questions, id);
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

            QuestionMeda { media: media.clone() }

            QuestionFormat { format: format.clone() }
        }

        ol {
            sub_questions.read().iter().enumerate().map(|(i,question)| {
                rsx!(
                    li {
                        key: "{question.id}",
                        QuestionUI { 
                            question_data: sub_questions.clone(),
                            question: question.clone(),
                            parent_idx: i,
                        }
                        button {
                            onclick: move |_ev| {
                                sub_questions.with_mut(|questions| {
                                    questions.remove(i);
                                });
                            },
                            "- remove question with id of {question.id.clone()}"
                        }
                    }
                )
            })
        }

        button {
            onclick: move |_ev| {
                counter += 1;
                sub_questions.with_mut(|questions| {
                    questions.push(Question::new_default(*counter.get()));
                });
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
    id: usize
}

#[derive(Props, PartialEq)]
struct QuestionUIProps{
    question_data: UseRef<Vec<Question>>,
    question: Question,
    parent_idx: usize,
}

impl Question {
    fn new(title: String,media: Option<String>,format: QuestionFormat,sub_questions: Vec<Question>, id: usize) -> Self {
        Question { title, media, format, sub_questions, id}
    }

    fn new_default(id:usize) -> Self {
        use QuestionFormat::*;
        Question { title: "New Question".into(), media: None, format: ShortAnswer, sub_questions: Vec::new(), id}
    } 
}
