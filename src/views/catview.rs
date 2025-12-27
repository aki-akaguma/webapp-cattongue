use crate::OverlaySpinner;
use dioxus::prelude::*;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct CatApi {
    id: String,
    url: String,
    width: i64,
    height: i64,
}

#[component]
pub fn CatView() -> Element {
    let mut is_loading = use_signal(|| false);
    let mut img_src = use_resource(move || async move {
        is_loading.set(true);
        //let r = reqwest::get("https://aws.random.cat/meow")
        let r = reqwest::get("https://api.thecatapi.com/v1/images/search")
            .await
            .unwrap()
            .json::<Vec<CatApi>>()
            .await
            .unwrap()[0]
            .url
            .clone();
        is_loading.set(false);
        r
    });
    //
    rsx! {
        div { id: "catview",
            img { src: img_src.cloned().unwrap_or_default() }
        }
        div { id: "buttons",
            button { onclick: move |_| img_src.restart(), id: "skip", "skip" }
            button {
                onclick: move |_| async move {
                    let current = img_src.cloned().unwrap();
                    img_src.restart();
                    _ = crate::backend::save_cat(current).await;
                },
                id: "save",
                "save!"
            }
        }
        if *is_loading.read() {
            OverlaySpinner {}
        }
    }
}
