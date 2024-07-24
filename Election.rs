use yew::prelude::*;
use yew::services::ConsoleService;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, PartialEq, Properties)]
pub struct CreateElectionProps {
    pub client: Client,
}

#[function_component(CreateElection)]
pub fn create_election(props: &CreateElectionProps) -> Html {
    let description = use_state(|| String::new());
    let oninput = {
        let description = description.clone();
        Callback::from(move |e: InputData| description.set(e.value))
    };

    let onclick = {
        let description = description.clone();
        let client = props.client.clone();
        Callback::from(move |_| {
            let description = description.clone();
            let client = client.clone();
            spawn_local(async move {
                // Logique pour appeler le smart contract et créer l'élection
                let res = client.post("https://your-aptos-endpoint/v1/create_election")
                    .json(&serde_json::json!({ "description": *description }))
                    .send()
                    .await;
                match res {
                    Ok(response) => ConsoleService::log(&format!("Election created: {:?}", response)),
                    Err(err) => ConsoleService::log(&format!("Error: {:?}", err)),
                }
            });
        })
    };

    html! {
        <div>
            <h2>{ "Create Election" }</h2>
            <input type="text" value={(*description).clone()} oninput={oninput} />
            <button onclick={onclick}>{ "Create" }</button>
        </div>
    }
}
