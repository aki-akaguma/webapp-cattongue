use crate::OverlaySpinner;
use async_sleep_aki::async_sleep;
use async_sleep_aki::{postponed_call, PostponedCall};
use dioxus::prelude::*;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct CatApi {
    id: String,
    url: String,
    width: i64,
    height: i64,
}

async fn button_enable(btn_id: &str, enabled: bool) {
    let js = format!(
        concat!(
            "{{const elem = document.getElementById(\"{}\");",
            "if (elem) {{elem.disabled = {};return \"ok\";}}",
            "else {{return \"err: not found: '{}'\";}}",
            "}}"
        ),
        btn_id, !enabled, btn_id
    );
    let _v = document::eval(&js).await.unwrap();
}

async fn check_complete(mut is_loading: Signal<bool>, mut postponed: Signal<PostponedCall>) {
    async_sleep(400).await;
    // read the complete property on javascript
    let js: &str = concat!(
        r#"{"#,
        r#"const elem = document.getElementById('catimg'); "#,
        r#"if (elem) {"#,
        r#"  return elem.complete; "#,
        r#"} else {"#,
        r#"  return 'not found #catimg';"#,
        r#"};"#,
        r#"}"#,
    );
    loop {
        let v = document::eval(js).await.unwrap();
        let s = v.to_string();
        if s == "true" {
            //dioxus::logger::tracing::debug!("img elem: '{s:?}'");
            if *is_loading.read() {
                is_loading.set(false);
                dioxus::logger::tracing::debug!("set is_loading: false");
            } else {
                dioxus::logger::tracing::debug!("already is_loading: false");
            }
            break;
        } else {
            //dioxus::logger::tracing::debug!("img elem: '{s:?}'");
            async_sleep(50).await;
        }
        if !*is_loading.read() {
            dioxus::logger::tracing::debug!("is_loading: false");
            break;
        }
    }
    let a = postponed_call(10, move || {});
    let _ = postponed.replace(a);
    button_enable("skip", true).await;
    button_enable("save", true).await;
}

/// the component of the `Cat page`
#[component]
pub fn CatView() -> Element {
    let mut is_loading = use_signal(|| false);
    let mut loading_count = use_signal(|| 0i64);
    let mut postponed = use_signal(|| postponed_call(10, move || {}));
    let mut img_src = use_resource(move || async move {
        is_loading.set(true);
        dioxus::logger::tracing::debug!("set is_loading: true");
        loading_count += 1;
        //let url = "https://aws.random.cat/meow";
        let url = "https://api.thecatapi.com/v1/images/search";
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
                let a = postponed_call(3000, move || {
                    dioxus::logger::tracing::debug!("postponed: call");
                    if *is_loading.read() {
                        is_loading.set(false);
                        let a = postponed_call(10, move || {});
                        let _ = postponed.replace(a);
                        dioxus::logger::tracing::debug!("postponed: set is_loading: false");
                    }
                });
                let _ = postponed.replace(a);
                spawn(check_complete(is_loading, postponed));
                r1
            }
        };
        loading_count -= 1;
        if *loading_count.read() > 0 {
            dioxus::logger::tracing::info!("loading_count: '{}'", *loading_count.read());
        }
        r
    });

    rsx! {
        div { id: "catview",
            img {
                id: "catimg",
                src: img_src.cloned().unwrap_or_default(),
            }
        }
        div { id: "buttons",
            button {
                onclick: move |_| async move {
                    button_enable("skip", false).await;
                    button_enable("save", false).await;
                    img_src.restart();
                },
                id: "skip",
                "skip"
            }
            button {
                onclick: move |_| async move {
                    button_enable("skip", false).await;
                    button_enable("save", false).await;
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
