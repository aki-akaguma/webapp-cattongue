use dioxus::prelude::*;

use components::*;
use views::{CatView, Favorites};

mod backend;
mod components;
mod views;

const FAVICON: Asset = asset!("/assets/favicon.ico");
#[cfg(not(feature = "inline_style"))]
const MAIN_CSS: Asset = asset!("/assets/main.css");
#[cfg(feature = "inline_style")]
const MAIN_CSS: &str = const_css_minify::minify!("../assets/main.css");

fn main() {
    /*
    #[cfg(not(feature = "server"))]
    {
        let backend_url = "https://hot-dog.fly.dev";
        dioxus_fullstack::set_server_url(backend_url);
    }
    */

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        //document::Link { rel: "stylesheet", href: MAIN_CSS }
        MyStyle {}

        Router::<Route> {}
    }
}

#[cfg(not(feature = "inline_style"))]
#[component]
fn MyStyle() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
    }
}

#[cfg(feature = "inline_style")]
#[component]
fn MyStyle() -> Element {
    rsx! {
        style { "{MAIN_CSS}" }
    }
}

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
