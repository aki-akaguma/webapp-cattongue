use browserinfocm::browserinfo::{BroInfo, Browser};
use browserinfocm::BrowserInfoCm;
use dioxus::prelude::*;
use std::time::Duration;

/// the component of browser information
#[component]
pub fn Info() -> Element {
    // browser info
    let broinfo_sig = use_signal(BroInfo::default);
    let browser_sig = use_signal(Browser::default);
    let bicmid_sig = use_signal(String::new);
    let user_sig = use_signal(String::new);

    //let brg = browser_sig.read().clone();
    //let bim = broinfo_sig.read().clone();
    /*
    let bicmid_s = bicmid_sig.read().clone();
    let user_s = user_sig.read().clone();
    */
    let mut check_session_sig = use_signal(|| false);
    /*
    let check_s = check_session_sig.read().to_string();
    */

    use_effect(move || {
        spawn(async move {
            let bicmid = {
                let mut bicmid: String;
                loop {
                    bicmid = bicmid_sig.read().clone();
                    if bicmid.is_empty() {
                        crate::async_sleep(Duration::from_millis(1)).await;
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
        BrowserInfoCm {
            broinfo: broinfo_sig,
            browser: browser_sig,
            bicmid: bicmid_sig,
            user: user_sig,
        }
        {}
    }
}
