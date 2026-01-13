use crate::OverlaySpinner;
use async_sleep_aki::{async_sleep, delayed_call};
use dioxus::prelude::*;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct CatApi {
    id: String,
    url: String,
    width: i64,
    height: i64,
}

async fn check_complete(mut is_loading: Signal<bool>) {
    async_sleep(400).await;
    let js: &str = concat!(
        r#"function img_complete(elem_id) {"#,
        r#" const elem = document.getElementById(elem_id); "#,
        r#" if (elem) { "#,
        r#"  return elem.complete; "#,
        r#" } else {"#,
        r#"  return 'not found #catimg';"#,
        r#" }"#,
        r#"}"#,
        r#"return img_complete('catimg');"#
    );
    loop {
        let v = document::eval(js).await.unwrap();
        let s = v.to_string();
        if s == "true" {
            async_sleep(200).await;
            if *is_loading.read() {
                is_loading.set(false);
            }
            break;
        } else {
            dioxus_logger::tracing::debug!("img elem: '{s:?}'");
            async_sleep(100).await;
        }
        if !*is_loading.read() {
            break;
        }
    }
}

/// the component of the `Cat page`
#[component]
pub fn CatView() -> Element {
    let mut is_loading = use_signal(|| false);
    let mut loading_count = use_signal(|| 0i64);
    let mut img_src = use_resource(move || async move {
        is_loading.set(true);
        loading_count += 1;
        //let url = "https://aws.random.cat/meow";
        let url = "https://api.thecatapi.com/v1/images/search";
        let resp = reqwest::get(url).await;
        let r = if let Err(_e) = resp {
            dioxus_logger::tracing::info!("error: {_e}");
            is_loading.set(false);
            "".to_string()
        } else {
            let body = resp.unwrap();
            let r1 = body.json::<Vec<CatApi>>().await.unwrap()[0].url.clone();
            spawn(async move {
                async_sleep(20).await;
                spawn(delayed_call(2000, async move {
                    if *is_loading.read() {
                        is_loading.set(false);
                    }
                }));
                spawn(check_complete(is_loading));
            });
            r1
        };
        loading_count -= 1;
        if *loading_count.read() > 0 {
            dioxus_logger::tracing::info!("loading_count: '{}'", *loading_count.read());
        }
        r
    });

    rsx! {
        div { id: "catview",
            img { id: "catimg", src: img_src.cloned().unwrap_or_default() }
        }
        div { id: "buttons",
            button { onclick: move |_| img_src.restart(), id: "skip", "skip" }
            button {
                onclick: move |_| async move {
                    let current = img_src.cloned().unwrap();
                    img_src.restart();
                    _ = crate::backends::save_cat(current).await;
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
