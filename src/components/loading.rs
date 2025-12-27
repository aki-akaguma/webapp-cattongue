use dioxus::prelude::*;

#[component]
pub(crate) fn ChildrenOrLoading(children: Element) -> Element {
    rsx! {
        MyStyle {}
        SuspenseBoundary {
            fallback: |_| rsx! {
                div { class: "spinner-outer",
                    div { class: "spinner" }
                }
            },
            {children}
        }
    }
}

#[component]
pub(crate) fn OverlaySpinner() -> Element {
    rsx! {
        MyStyle {}
        div { class: "overlay",
            div { class: "spinner-outer",
                div { class: "spinner" }
            }
        }
    }
}

#[cfg(not(feature = "inline_style"))]
#[component]
fn MyStyle() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/loading.css") }
    }
}

#[cfg(feature = "inline_style")]
#[component]
fn MyStyle() -> Element {
    //const LOADING_CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/loading.css"));
    const LOADING_CSS: &str = const_css_minify::minify!("../../assets/loading.css");
    rsx! {
        style { "{LOADING_CSS}" }
    }
}
