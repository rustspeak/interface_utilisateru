use yew::prelude::*;
use yew::services::ConsoleService;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, PartialEq, Properties)]
pub struct VoteProps {
    pub client: Client,
}

#[function_component(Vote)]
pub fn vote(props: &VoteProps) -> Html {
    let election_id = use_state(|| String::new());
    let choice = use_state(|| String::new());

    let oninput_id = {
        let election_id = election_id.clone();
        Callback::from(move |e: InputData| election_id.set(e.value))
    };

    let oninput_choice = {
        let choice = choice.clone();
        Callback::from(move |e: InputData| choice.set(e.value))
    };

    let onclick = {
        let election_id = election_id.clone();
        let choice = choice.clone();
        let client = props.client.clone();
        Callback::from(move |_| {
            let election_id = election_id.clone();
            let choice = choice.clone();
            let client = client.clone();
            spawn_local(async move {
                // Logique pour appeler le smart contract et voter
                let res = client.post("https://your-aptos-endpoint/v1/vote")
                    .json(&serde_json::json!({ "election_id": *election_id, "choice": *choice }))
                    .send()
                    .await;
                match res {
                    Ok(response) => ConsoleService::log(&format!("Voted successfully: {:?}", response)),
                    Err(err) => ConsoleService::log(&format!("Error: {:?}", err)),
                }
            });
        })
    };

    html! {
        <div>
            <h2>{ "Vote" }</h2>
            <input type="text" value={(*election_id).clone()} oninput={oninput_id} placeholder="Election ID" />
            <input type="text" value={(*choice).clone()} oninput={oninput_choice} placeholder="Choice" />
            <button onclick={onclick}>{ "Vote" }</button>
        </div>
    }
}
