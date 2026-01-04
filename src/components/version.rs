use dioxus::prelude::*;

/// the component of version information
#[component]
pub fn Version() -> Element {
    let pkg_version = env!("CARGO_PKG_VERSION");
    rsx! {
        div { class: "version", "ver: {pkg_version}" }
    }
}
