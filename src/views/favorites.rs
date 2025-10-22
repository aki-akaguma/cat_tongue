use crate::OverlaySpinner;
use dioxus::prelude::*;
use dioxus_fullstack::Loader;

#[component]
pub fn Favorites() -> Element {
    let mut is_loading = use_signal(|| false);
    let favorites = use_loader(move || async move {
        is_loading.set(true);
        let r = crate::backend::list_cats().await;
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
            div { id: "favorites-container",
                for (id, url) in favorites.cloned() {
                    FavoriteDog { id:id, url: url, favorites: favorites, is_loading: is_loading }
                }
            }
        }
        if *is_loading.read() {
            OverlaySpinner {}
        }
    }
}

#[component]
pub fn FavoriteDog(
    id: usize,
    url: String,
    favorites: Loader<Vec<(usize, String)>>,
    is_loading: Signal<bool>,
) -> Element {
    // Render a div for each photo using the cat's ID as the list key
    rsx! {
        div {
            key: "{id}",
            class: "favorite-cat",
            img { src: "{url}" }
            button { onclick: move |_| async move {
                is_loading.set(true);
                _ = crate::backend::delete_cat(id).await;
                favorites.restart();
            }, id: "delete", "🚫" }
        }
    }
}
