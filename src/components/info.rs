use browserinfocm::browserinfo::{BroInfo, Browser};
use browserinfocm::BrowserInfoCm;
use dioxus::prelude::*;

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
