//! The page GitHub Pages serves for any path that has no file.

use dioxus::prelude::*;

use crate::chrome::page;

/// The not-found page's `body`.
pub(crate) fn body() -> Element {
    page(rsx! {
        main { id: "content", class: "center", "data-max": "md",
            div { class: "stack", "data-gap": "md",
                h1 { "Not found" }
                p { "There is nothing at this address. It may have moved, or never existed." }
                div { class: "cluster", "data-gap": "sm",
                    a { class: "button", href: "/", "Home" }
                    a { class: "button", href: "/gantry/", "data-emphasis": "medium", "Gantry docs" }
                }
            }
        }
    })
}
