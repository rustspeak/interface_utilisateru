use yew::prelude::*;
use yew::services::ConsoleService;
use yew_router::prelude::*;
use reqwest::Client;

mod components;
use components::{Home, Election, Vote, Results};

#[function_component(App)]
fn app() -> Html {
    let client = Client::new();

    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

#[derive(Switch, Debug, Clone)]
enum Route {
    #[to = "/create"]
    CreateElection,
    #[to = "/vote"]
    Vote,
    #[to = "/results"]
    Results,
    #[to = "/"]
    Home,
}

fn switch(routes: &Route) -> Html {
    let client = Client::new();
    match routes {
        Route::CreateElection => html! { <CreateElection client={client} /> },
        Route::Vote => html! { <Vote client={client} /> },
        Route::Results => html! { <Results client={client} /> },
        Route::Home => html! { <Home /> },
    }
}

fn main() {
    yew::start_app::<App>();
}
