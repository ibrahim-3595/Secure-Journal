mod api;
mod components;
mod models;
mod pages;
mod state;

use dioxus::prelude::*;
use crate::pages::login::Login;
use crate::pages::signup::Signup;
use crate::pages::entries::Entries;
use crate::pages::new_entry::NewEntry;
use crate::pages::home::Home;
use state::AppState;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Login {},
    #[route("/signup")]
    Signup {},
    #[route("/home")]
    Home {},
    #[route("/entries")]
    Entries {},
    #[route("/new-entry")]
    NewEntry {},
}

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(AppState::default()));
    
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}