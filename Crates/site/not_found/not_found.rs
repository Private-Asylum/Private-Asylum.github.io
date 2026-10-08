//! The page GitHub Pages serves for any path that has no file.

use dioxus::prelude::*;

use pvas_web_shared::components::{Intro, NavLink};

use crate::chrome::page;

/// The not-found page's `body`.
pub(crate) fn body() -> Element {
    page(rsx! {
        main { id: "content",
            Intro {
                eyebrow: "404",
                headline: "Not found.",
                lede: "There is nothing at this address. It may have moved, or never existed.",
                actions: vec![NavLink::new("Home", "/"), NavLink::new("Gantry docs", "/gantry/")],
            }
        }
    })
}
