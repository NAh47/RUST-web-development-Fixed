use gloo_net::http;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[derive(Deserialize, Clone, Debug)]
struct Question {
    id: String,
    header: String,
    body: String,
    categories: Vec<String>,
}

#[function_component(App)]
fn app() -> Html {
    let questions = use_state(|| vec![]);

    {
        let questions = questions.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let fetched_questions: Vec<Question> = http::Request::get("http://localhost:3000/questions")
                    .send()
                    .await
                    .expect("Failed to fetch questions")
                    .json()
                    .await
                    .expect("Failed to parse questions");
                questions.set(fetched_questions);
            });
            || ()
        });
    }

    html! {
        <>
            <h1>{ "Questions" }</h1>
            <ul>
                { for questions.iter().map(|question| html! {
                    <li key={question.id.clone()}>
                        <h2>{ &question.header }</h2>
                        <p>{ &question.body }</p>
                    </li>
                }) }
            </ul>
        </>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    yew::Renderer::<App>::new().render();
}
