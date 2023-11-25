use dioxus::prelude::*;

#[derive(PartialEq, Debug, Clone)]
pub enum QuestionFormat {
    ShortAnswer,
    MultipleChoice(Vec<String>),
    Justify,
    TrueOrFalse(u8, Vec<String>)  
}

#[derive(Props, PartialEq)]
pub struct QuestionFormatWrapper {
    format: UseRef<QuestionFormat>
}

impl From<String> for QuestionFormat {
    fn from(value: String) -> Self {
        use QuestionFormat::*;
        match value.as_str() {
            "0" => {
                ShortAnswer
            },
            "1" => {
                MultipleChoice(vec!["New Option".to_string()])
            },
            "2" => {
                Justify
            },
            "3" => {
                TrueOrFalse(1, vec!["New Option".to_string()])
            },

            _ => panic!("UNIMPLEMENTED FORMAT")
        }
    }
}

pub fn QuestionFormat(cx: Scope<QuestionFormatWrapper>) -> Element {
    
    let format = &cx.props.format;
    let choices = use_ref(cx, || {vec![(0,"New Option".to_string())]});
    let max_answers = use_state(cx, || 1);
    let mut key_counter = use_state(cx, || 0);

    use_effect(cx, choices, |choices| {
        to_owned![format,choices];
        async move {
            use QuestionFormat::*;
            format.with_mut( |format_value| {
                match format_value {
                    MultipleChoice(_) => {
                        let clean_choices = choices.with(|choices| {
                            let mut new_choices = vec![];
                            choices.iter().for_each(|c| {
                                new_choices.push(c.1.clone());
                            });
                            new_choices
                        });
                        *format_value = MultipleChoice(clean_choices);
                    },
                    TrueOrFalse(i,_) => {
                        let clean_choices = choices.with(|choices| {
                            let mut new_choices = vec![];
                            choices.iter().for_each(|c| {
                                new_choices.push(c.1.clone());
                            });
                            new_choices
                        });
                        *format_value = TrueOrFalse(*i,clean_choices);
                    }
                    _ => {
                        ()
                    }
                }
            })
        }
    });

    use_effect(cx, max_answers, |max_answers| {
        to_owned![format,max_answers];
        async move {
            format.with_mut(|format| {
                match format {
                    QuestionFormat::TrueOrFalse(_, choices) => {
                        *format = QuestionFormat::TrueOrFalse(*max_answers.get(), choices.to_owned())
                    },
                    _ => {
                        ()
                    }
                }
            });
        }
    });

    render!(
        select {
            oninput: |ev| {
                format.with_mut(|cur| {
                    *cur = QuestionFormat::from(ev.data.value.clone());
                });
                key_counter.set(0);
                choices.set(vec![(key_counter.get().to_owned(),"New Option".to_string())]);
                max_answers.set(1);
            },

            option {
                value: 0,
                "Short Answer Question"
            },
            option {
                value: 1,
                "Multiple Choice Question"
            },
            option {
                value: 2,
                "Justification Question"
            },
            option {
                value: 3,
                "True or False Question"
            }
        }

        "{key_counter}"

        format.with(|format_render| {
            use QuestionFormat::*;
            match format_render.clone() {
                ShortAnswer => {
                    rsx!(
                        div {
                            style: r#"
                                width: 70%;
                                height: 150px;
                                border: 1px solid black;
                                border-radius: 15%:
                            "#
                        }
                    )
                },
                MultipleChoice(_) => {
                   rsx!(
                    ol {
                        choices.read().iter().enumerate().map(|(i,choice)| {
                            to_owned![choice];
                            rsx!(
                                li {
                                    input {
                                        value: "{choice.1}",
                                        oninput: move |ev| {
                                            choices.with_mut(|new_choices| {
                                                new_choices[i] = (choice.0, ev.data.value.clone());
                                            })
                                        }
                                    }

                                    button {
                                        onclick: move |_| {
                                            choices.with_mut(|choices| {
                                                choices.remove(i);
                                            })
                                        },
                                        disabled: choices.with(|choices| {
                                            1 >= choices.len()
                                        }),
                                        "X"
                                    }
                                }
                            )
                        })
                    }
                    
                    button {
                        onclick: move |_| {
                            key_counter += 1;
                            choices.with_mut(|choices| choices.push((key_counter.get().to_owned(),"New Option".to_string())));
                        },
                        "+ add choice"
                    }
                   ) 
                },
                Justify => {
                    rsx!(
                        label { "True or False, Justify Answer" }
                        input { r#type: "checkbox" }
                        div {
                            style: r#"
                                width: 70%;
                                height: 150px;
                                border: 1px solid black;
                                border-radius: 15%:
                            "#
                        }
                    )
                },
                TrueOrFalse(_,_) => {
                         rsx!(
                    ol {
                        choices.read().iter().enumerate().map(|(i,choice)| {
                            to_owned![choice];
                            rsx!(
                                li {
                                    input {
                                        value: "{choice.1}",
                                        oninput: move |ev| {
                                            choices.with_mut(|new_choices| {
                                                new_choices[i] = (choice.0, ev.data.value.clone());
                                            })
                                        }
                                    }

                                    button {
                                        onclick: move |_| {
                                            choices.with_mut(|choices| {
                                                choices.remove(i);
                                            })
                                        },
                                        disabled: choices.with(|choices| {
                                            1 >= choices.len()
                                        }),
                                        "X"
                                    }
                                }
                            )
                        })
                    }
                    
                    button {
                        onclick: move |_| {
                            key_counter += 1;
                            choices.with_mut(|choices| choices.push((key_counter.get().to_owned(),"New Option".to_string())));
                        },
                        "+ add choice"
                    }
                    div {
                        for _i in 0..*max_answers.get() {
                            rsx!(
                                input {
                                    r#type: "checkbox"
                                }
                            )
                        }
                    }


                    label {
                        "Type in the number of correct answers"
                    }
                    input {
                        r#type: "number",
                        max: choices.with(|choices| {
                            choices.len() as i64
                        }),
                        min: 1,
                        oninput: move |ev| {
                            let mut number_of_choices = ev.data.value.as_str().parse::<u8>().unwrap_or(1);
                            if number_of_choices as usize > choices.with(|choices| choices.len()) {
                                number_of_choices = 1;
                            }
                            max_answers.set(number_of_choices);
                        },
                    }
                   ) 
                }
            }
        })
    )
}