use dioxus::prelude::*;

#[component]
pub(crate) fn ChildrenOrLoading(children: Element) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/loading.css") }
        SuspenseBoundary {
            fallback: |_| rsx! {
                div { class: "spinner-outer",
                    div { class: "spinner", }
                }
            },
            {children}
        }
    }
}

#[component]
pub(crate) fn OverlaySpinner() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/loading.css") }
        div { class: "overlay",
            div { class: "spinner-outer",
                div { class: "spinner",
                }
            }
        }
    }
}
