use yew::prelude::*;
use yew::services::ConsoleService;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, PartialEq, Properties)]
pub struct ResultsProps {
    pub client: Client,
}

#[derive(Deserialize, Debug, Clone)]
struct Vote {
    voter: String,
    choice: u8,
}

#[derive(Deserialize, Debug, Clone)]
struct ElectionResults {
    id: u64,
    description: String,
    votes: Vec<Vote>,
}

#[function_component(Results)]
pub fn results(props: &ResultsProps) -> Html {
    let results = use_state(|| vec![]);

    {
        let results = results.clone();
        let client = props.client.clone();
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                let res = client.get("https://your-aptos-endpoint/v1/results")
                    .send()
                    .await
                    .unwrap()
                    .json::<Vec<ElectionResults>>()
                    .await;
                match res {
                    Ok(response) => results.set(response),
                    Err(err) => ConsoleService::log(&format!("Error: {:?}", err)),
                }
            });
            || ()
        }, ());
    }

    html! {
        <div>
            <h2>{ "Results" }</h2>
            { for results.iter().map(|result| html! {
                <div>
                    <h3>{ &result.description }</h3>
                    <ul>
                        { for result.votes.iter().map(|vote| html! {
                            <li>{ format!("Voter: {}, Choice: {}", vote.voter, vote.choice) }</li>
                        }) }
                    </ul>
                </div>
            }) }
        </div>
    }
}
