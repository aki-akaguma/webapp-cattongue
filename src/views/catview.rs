use crate::OverlaySpinner;
use async_sleep_aki::postponed_call;
use dioxus::prelude::*;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct CatApi {
    id: String,
    url: String,
    width: i64,
    height: i64,
}

/// the component of the `Cat page`
#[component]
pub fn CatView() -> Element {
    let mut is_loading = use_signal(|| false);
    let mut error_msg = use_signal(|| None::<String>);
    let mut loading_count = use_signal(|| 0i64);
    let mut postponed = use_signal(|| postponed_call(10, move || {}));

    let mut img_src = use_resource(move || async move {
        is_loading.set(true);
        dioxus::logger::tracing::debug!("set is_loading: true");
        error_msg.set(None);
        loading_count += 1;

        let url = "https://api.thecatapi.com/v1/images/search";

        // Receive the result of asynchronous processing as Result
        let fetch_result = async {
            let resp = reqwest::get(url)
                .await
                .map_err(|e| format!("Network error: {}", e))?;
            let body = resp
                .json::<Vec<CatApi>>()
                .await
                .map_err(|e| format!("JSON parse error: {}", e))?;
            body.first()
                .map(|c| c.url.clone())
                .ok_or_else(|| "No cat image found in the response".to_string())
        }
        .await;

        let r = match fetch_result {
            Ok(r1) => {
                // Timeout setting if it cannot be loaded after 3 seconds
                let a = postponed_call(3000, move || {
                    if *is_loading.read() {
                        is_loading.set(false);
                        dioxus::logger::tracing::debug!("timeout: set is_loading: false");
                    }
                });
                let _ = postponed.replace(a);
                r1
            }
            Err(e) => {
                dioxus::logger::tracing::error!("{}", e);
                error_msg.set(Some(e));
                is_loading.set(false);
                "".to_string()
            }
        };

        loading_count -= 1;
        r
    });

    /*
        let resp = reqwest::get(url).await;
        let r = if let Err(_e) = resp {
            dioxus::logger::tracing::info!("error: {_e}");
            is_loading.set(false);
            "".to_string()
        } else {
            let body = resp.unwrap();
            let r = body.json::<Vec<CatApi>>().await;
            if let Err(_e) = r {
                dioxus::logger::tracing::info!("error: {_e}");
                is_loading.set(false);
                "".to_string()
            } else {
                let r1 = r.unwrap()[0].url.clone();
                // 3秒経っても読み込めない場合のタイムアウト
                let a = postponed_call(3000, move || {
                    if *is_loading.read() {
                        is_loading.set(false);
                        dioxus::logger::tracing::debug!("timeout: set is_loading: false");
                    }
                });
                let _ = postponed.replace(a);
                r1
            }
        };

        loading_count -= 1;
        if *loading_count.read() > 0 {
            dioxus::logger::tracing::info!("loading_count: '{}'", *loading_count.read());
        }
        r
    });
    */

    rsx! {
        div { id: "catview",
            if let Some(err) = error_msg.read().as_ref() {
                div { class: "error-box",
                    p { "{err}" }
                    button { onclick: move |_| img_src.restart(), "Retry" }
                }
            } else {
                img {
                    id: "catimg",
                    src: img_src.cloned().unwrap_or_default(),
                    onload: move |_| {
                        is_loading.set(false);
                        dioxus::logger::tracing::debug!("img onload: set is_loading: false");
                        // Cancel timer for timeout (overwrite with immediate execution)
                        let _ = postponed.replace(postponed_call(10, move || {}));
                    },
                }
            }
        }
        div { id: "buttons",
            button {
                // Automatically disabled in conjunction with is_loading signal
                disabled: *is_loading.read(),
                onclick: move |_| async move {
                    img_src.restart();
                },
                id: "skip",
                "skip"
            }
            button {
                // Disable save button when there is no image or while loading
                disabled: *is_loading.read() || img_src.read().as_ref().is_none_or(|s| s.is_empty()),
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
