use gloo_console::log;
use gloo_net::http;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use std::ops::Deref;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
struct Question {
    id: String,
    header: String,
    body: String,
    categories: Option<Vec<String>>,
}

#[function_component(App)]
fn app() -> Html {
    let questions = use_state(|| vec![]);
    let new_question = use_state(|| Question {
        id: "".into(),
        header: "".into(),
        body: "".into(),
        categories: None,
    });
    let update_question = use_state(|| Question {
        id: "".into(),
        header: "".into(),
        body: "".into(),
        categories: None,
    });
    let fetch_id: UseStateHandle<String> = use_state(|| "".into());
    let delete_id: UseStateHandle<String> = use_state(|| "".into());
    let show_questions = use_state(|| false);

    let on_input_change = {
        let new_question = new_question.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = input.value();
            let name = input.name();
            new_question.set(Question {
                id: if name == "id" { value.clone() } else { new_question.id.clone() },
                header: if name == "header" { value.clone() } else { new_question.header.clone() },
                body: if name == "body" { value.clone() } else { new_question.body.clone() },
                categories: None,
            });
        })
    };

    let on_update_input_change = {
        let update_question = update_question.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = input.value();
            let name = input.name();
            update_question.set(Question {
                id: if name == "id" { value.clone() } else { update_question.id.clone() },
                header: if name == "header" { value.clone() } else { update_question.header.clone() },
                body: if name == "body" { value.clone() } else { update_question.body.clone() },
                categories: None,
            });
        })
    };

    let on_create = {
        let new_question = new_question.clone();
        let questions = questions.clone();
        Callback::from(move |_| {
            let question = new_question.deref().clone();
            let questions = questions.clone();
            spawn_local(async move {
                match http::Request::post("http://localhost:3000/questions")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&question).unwrap())
                    .send()
                    .await {
                        Ok(response) => {
                            log!("Create response:", format!("{:?}", response));
                            questions.set(vec![question]);
                        }
                        Err(err) => log!("Failed to create question:", err.to_string()),
                    }
            });
        })
    };

    let on_fetch_by_id = {
        let fetch_id = fetch_id.clone();
        let questions = questions.clone();
        Callback::from(move |_| {
            let id = fetch_id.deref().clone();
            let questions = questions.clone();
            spawn_local(async move {
                match http::Request::get(&format!("http://localhost:3000/questions/{}", id))
                    .send()
                    .await {
                        Ok(response) => {
                            match response.json::<Question>().await {
                                Ok(fetched_question) => {
                                    log!("Fetched question by ID:", format!("{:?}", fetched_question));
                                    questions.set(vec![fetched_question]);
                                }
                                Err(err) => log!("Failed to parse question by ID:", err.to_string()),
                            }
                        }
                        Err(err) => log!("Failed to fetch question by ID:", err.to_string()),
                    }
            });
        })
    };

    let on_update = {
        let update_question = update_question.clone();
        Callback::from(move |_| {
            let question = update_question.deref().clone();
            let id = question.id.clone();
            spawn_local(async move {
                match http::Request::put(&format!("http://localhost:3000/questions/{}", id))
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&question).unwrap())
                    .send()
                    .await {
                        Ok(response) => log!("Update response:", format!("{:?}", response)),
                        Err(err) => log!("Failed to update question:", err.to_string()),
                    }
            });
        })
    };

    let on_delete = {
        let delete_id = delete_id.clone();
        let questions = questions.clone();
        Callback::from(move |_| {
            let id = delete_id.deref().clone();
            let questions = questions.clone();
            spawn_local(async move {
                match http::Request::delete(&format!("http://localhost:3000/questions/{}", id))
                    .send()
                    .await {
                        Ok(response) => {
                            log!("Delete response:", format!("{:?}", response));
                            let updated_questions: Vec<Question> = questions.deref().iter().filter(|q| q.id != id).cloned().collect();
                            questions.set(updated_questions);
                        }
                        Err(err) => log!("Failed to delete question:", err.to_string()),
                    }
            });
        })
    };

    let on_fetch_all = {
        let questions = questions.clone();
        let show_questions = show_questions.clone();
        Callback::from(move |_| {
            if *show_questions {
                show_questions.set(false);
                questions.set(vec![]);
            } else {
                let questions = questions.clone();
                let show_questions = show_questions.clone();
                spawn_local(async move {
                    match http::Request::get("http://localhost:3000/questions")
                        .send()
                        .await {
                            Ok(response) => {
                                match response.json::<Vec<Question>>().await {
                                    Ok(fetched_questions) => {
                                        log!("Fetched questions:", format!("{:?}", fetched_questions));
                                        questions.set(fetched_questions);
                                        show_questions.set(true);
                                    }
                                    Err(err) => log!("Failed to parse questions:", err.to_string()),
                                }
                            }
                            Err(err) => log!("Failed to fetch questions:", err.to_string()),
                        }
                });
            }
        })
    };

    html! {
        <div class="container">
            <h1>{ "Rust Questions" }</h1>
            <button onclick={on_fetch_all}>{ if *show_questions { "Hide All" } else { "Fetch All" } }</button>
            <div class="flex">
                <div class="panel">
                    <h2>{ "Questions" }</h2>
                    <ul>
                        { for questions.iter().map(|question| html! {
                            <li key={question.id.clone()}>
                                <h3>{ format!("ID: {}, Header: {}", question.id, question.header) }</h3>
                                <p>{ &question.body }</p>
                                <p>{ format!("{:?}", &question.categories) }</p>
                            </li>
                        }) }
                    </ul>
                </div>
                <div class="panel">
                    <h2>{ "Create Question" }</h2>
                    <input type="text" placeholder="ID" name="id" value={new_question.id.clone()} oninput={on_input_change.clone()} />
                    <input type="text" placeholder="Header" name="header" value={new_question.header.clone()} oninput={on_input_change.clone()} />
                    <textarea placeholder="Body" name="body" value={new_question.body.clone()} oninput={on_input_change.clone()}></textarea>
                    <button onclick={on_create}>{ "Create" }</button>

                    <h2>{ "Fetch Question by ID" }</h2>
                    <input type="text" placeholder="ID" value={fetch_id.deref().clone()} oninput={Callback::from(move |e: InputEvent| {
                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                        fetch_id.set(input.value());
                    })} />
                    <button onclick={on_fetch_by_id}>{ "Fetch" }</button>

                    <h2>{ "Update Question" }</h2>
                    <input type="text" placeholder="ID" name="id" value={update_question.id.clone()} oninput={on_update_input_change.clone()} />
                    <input type="text" placeholder="Header" name="header" value={update_question.header.clone()} oninput={on_update_input_change.clone()} />
                    <textarea placeholder="Body" name="body" value={update_question.body.clone()} oninput={on_update_input_change.clone()}></textarea>
                    <button onclick={on_update}>{ "Update" }</button>

                    <h2>{ "Delete Question" }</h2>
                    <input type="text" placeholder="ID" value={delete_id.deref().clone()} oninput={Callback::from(move |e: InputEvent| {
                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                        delete_id.set(input.value());
                    })} />
                    <button onclick={on_delete}>{ "Delete" }</button>
                </div>
            </div>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    yew::Renderer::<App>::new().render();
}
