use dioxus::prelude::*;
#[cfg(all(not(debug_assertions), feature = "desktop"))]
use dioxus_desktop::{Config, WindowBuilder};

use components::*;
use views::{CatView, Favorites};

mod backends;
mod components;
mod views;

fn main() {
    // You can set the ports and IP manually with env vars:
    //   server launch:
    //     IP="0.0.0.0" PORT=8080 ./server

    // You can supplement panic on  firefox browser.
    #[cfg(feature = "web")]
    console_error_panic_hook::set_once();

    #[cfg(not(debug_assertions))]
    let level = dioxus_logger::tracing::Level::INFO;
    #[cfg(debug_assertions)]
    let level = dioxus_logger::tracing::Level::DEBUG;
    dioxus_logger::init(level).expect("failed to init logger");

    // In the case of release desktop and release mobile,
    // connect backend calls to public api
    #[cfg(not(debug_assertions))]
    #[cfg(any(feature = "desktop", feature = "mobile"))]
    {
        // Specify the URL that previously delpoyed the public webapp.
        // This webapp was created with `dx bundle --web`.
        let backend_url = "https://aki.omusubi.org/cattongue";
        dioxus_fullstack::set_server_url(backend_url);
    }

    // In the case of only release desktop, set a window title
    #[cfg(not(feature = "server"))]
    #[cfg(all(not(debug_assertions), feature = "desktop"))]
    dioxus::LaunchBuilder::new()
        .with_cfg(
            Config::default().with_menu(None).with_window(
                WindowBuilder::new()
                    .with_maximized(false)
                    .with_title("Cat Tongue"),
            ),
        )
        .launch(App);

    // In the other case, simple launch app
    #[cfg(not(feature = "server"))]
    #[cfg(any(debug_assertions, not(feature = "desktop")))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async {
        let cookie_path = match std::env::var("DIOXUS_ASSET_ROOT") {
            Ok(s) => format!("/{s}"),
            Err(_e) => "/".to_string(),
        };
        dioxus_logger::tracing::info!("cookie_path: '{}'", &cookie_path);
        let session_layer = {
            use tower_sessions::{cookie::time::Duration, Expiry, SessionManagerLayer};
            let store = crate::backends::session_store().await.unwrap();
            SessionManagerLayer::new(store)
                .with_name("cttg.sid")
                .with_path(cookie_path)
                .with_secure(false) // https
                .with_always_save(false)
                .with_expiry(Expiry::OnInactivity(Duration::days(30)))
                .with_same_site(tower_sessions::cookie::SameSite::Lax)
        };
        //
        Ok(dioxus::server::router(App).layer(session_layer))
    })
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
#[cfg(not(feature = "inline_style"))]
const MAIN_CSS: Asset = asset!("/assets/main.css");
#[cfg(feature = "inline_style")]
const MAIN_CSS: &str = const_css_minify::minify!("../assets/main.css");

/// the component of dioxus `App`
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        MyStyle {}
        Info {}
        Router::<Route> {}
        Version {}
    }
}

/// the component of `main` style sheet
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
