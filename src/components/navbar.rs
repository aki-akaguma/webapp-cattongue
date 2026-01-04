use crate::Route;
use dioxus::prelude::*;

/// the component of navigation bar
#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { id: "title",
            Link { to: Route::CatView,
                h1 { "🐱 Cat's Tongue! 👅" }
            }
            {}
            Link { to: Route::Favorites {}, id: "heart", "♥️" }
        }
        Outlet::<Route> {}
    }
}
