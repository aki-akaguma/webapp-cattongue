use dioxus::prelude::*;

use components::*;
use views::{CatView, Favorites};

mod backend;
mod components;
mod views;

use components::Info;
use components::Version;

const FAVICON: Asset = asset!("/assets/favicon.ico");
#[cfg(not(feature = "inline_style"))]
const MAIN_CSS: Asset = asset!("/assets/main.css");
#[cfg(feature = "inline_style")]
const MAIN_CSS: &str = const_css_minify::minify!("../assets/main.css");

fn main() {
    // you can set the ports and IP manually with env vars:
    // server launch:
    // IP="0.0.0.0" PORT=8080 ./server

    #[cfg(feature = "web")]
    console_error_panic_hook::set_once();

    #[cfg(not(debug_assertions))]
    let level = dioxus_logger::tracing::Level::INFO;
    #[cfg(debug_assertions)]
    let level = dioxus_logger::tracing::Level::DEBUG;
    dioxus_logger::init(level).expect("failed to init logger");

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
        Info {}
        Router::<Route> {}
        Version {}
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
