use browserinfocm::BrowserInfoCm;
use browserinfocm::BrowserInfoState;
use dioxus::prelude::*;

/// the component of browser information
#[component]
pub fn Info() -> Element {
    // Signals for storing data gathered by BrowserInfoCm.
    let state_sig = use_signal(BrowserInfoState::default);

    //
    let mut check_session_sig = use_signal(|| false);

    use_effect(move || {
        spawn(async move {
            let bicmid = {
                let mut bicmid: String;
                loop {
                    bicmid = state_sig.read().bicmid.clone();
                    if bicmid.is_empty() {
                        async_sleep_aki::async_sleep(1).await;
                        continue;
                    }
                    break;
                }
                bicmid
            };
            let r = crate::backends::check_session(bicmid).await.unwrap();
            check_session_sig.set(r);
        });
    });

    rsx! {
        BrowserInfoCm { state: state_sig }
        {}
    }
}
