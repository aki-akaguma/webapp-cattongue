use crate::OverlaySpinner;
use dioxus::prelude::*;
use dioxus_fullstack::Loader;

#[component]
pub fn Favorites() -> Element {
    let mut is_loading = use_signal(|| false);
    let offset = use_signal(|| 0usize);
    let count_of_cats = use_loader(move || async move { crate::backend::count_of_cats().await })?;
    let favorites = use_loader(move || async move {
        is_loading.set(true);
        let r = crate::backend::list_cats(*offset.read()).await;
        is_loading.set(false);
        r
    })?;
    /*
    // Create a pending resource that resolves to the list of cats from the backend
    // Wait for the favorites list to resolve with `.suspend()`
    let favorites = use_resource(move || async move {
        is_loading.set(true);
        let r = crate::backend::list_cats().await;
        is_loading.set(false);
        r
    });
    let fav_sus = favorites.suspend()?;
    */

    rsx! {
        div { id: "favorites",
            div { id: "favorites-navi",
                div {
                    HandLeft {
                        offset,
                        count_of_cats,
                        favorites,
                        is_loading,
                    }
                    " {offset+1} / {count_of_cats} "
                    HandRight {
                        offset,
                        count_of_cats,
                        favorites,
                        is_loading,
                    }
                }
            }
            div { id: "favorites-container",
                for (id , url) in favorites.cloned() {
                    FavoriteDog {
                        id,
                        url,
                        count_of_cats,
                        favorites,
                        is_loading,
                    }
                }
            }
        }
        if *is_loading.read() {
            OverlaySpinner {}
        }
    }
}

#[component]
pub fn HandLeft(
    offset: Signal<usize>,
    count_of_cats: Loader<usize>,
    favorites: Loader<Vec<(usize, String)>>,
    is_loading: Signal<bool>,
) -> Element {
    rsx! {
        if *offset.read() >= 20 {
            button {
                onclick: move |_| async move {
                    is_loading.set(true);
                    let curr = *offset.read();
                    if curr >= 20 {
                        offset.set(curr - 20);
                    }
                    favorites.restart();
                },
                id: "handleft",
                "👈"
            }
        } else {
            button { disabled: true, id: "handleft", "🫷" }
        }
    }
}

#[component]
pub fn HandRight(
    offset: Signal<usize>,
    count_of_cats: Loader<usize>,
    favorites: Loader<Vec<(usize, String)>>,
    is_loading: Signal<bool>,
) -> Element {
    rsx! {
        if *offset.read() + 20 < *count_of_cats.read() {
            button {
                onclick: move |_| async move {
                    is_loading.set(true);
                    let curr = *offset.read();
                    offset.set(curr + 20);
                    favorites.restart();
                },
                id: "handright",
                "👉"
            }
        } else {
            button { disabled: true, id: "handright", "🫸" }
        }
    }
}

#[component]
pub fn FavoriteDog(
    id: usize,
    url: String,
    count_of_cats: Loader<usize>,
    favorites: Loader<Vec<(usize, String)>>,
    is_loading: Signal<bool>,
) -> Element {
    // Render a div for each photo using the cat's ID as the list key
    rsx! {
        div { key: "{id}", class: "favorite-cat",
            img { src: "{url}" }
            button {
                onclick: move |_| async move {
                    is_loading.set(true);
                    _ = crate::backend::delete_cat(id).await;
                    count_of_cats.restart();
                    favorites.restart();
                },
                id: "delete",
                "🚫"
            }
        }
    }
}
