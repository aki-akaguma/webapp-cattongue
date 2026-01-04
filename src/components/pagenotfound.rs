use dioxus::prelude::*;

/// the component of page not found
#[component]
pub fn PageNotFound(segments: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {segments:?}" }
    }
}
