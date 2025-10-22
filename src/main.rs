use dioxus::prelude::*;

use components::*;
use views::{CatView, Favorites};

mod backend;
mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    CatView,
    #[route("/favorites")]
    Favorites,
    // We can collect the segments of the URL into a Vec<String>
    #[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        //document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Stylesheet { href: MAIN_CSS }

        Router::<Route> {}
    }
}
