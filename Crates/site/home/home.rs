//! The home page: who we are, what we make, where the documentation is.

use dioxus::prelude::*;

use pvas_web_shared::components::{Card, Intro, NavLink};

use crate::{Availability, PRODUCTS};
use crate::chrome::page;

/// The home page's `body`.
pub(crate) fn body() -> Element {
    page(rsx! {
        main { id: "content", class: "stack", "data-gap": "3xl",
            Intro {
                eyebrow: "private-asylum",
                headline: "Tools for building worlds in Unreal Engine.",
                lede: "Private Asylum makes Unreal Engine plugins for content pipelines, \
                       planet-scale terrain and star systems, documented from the source they ship.",
                actions: vec![
                    NavLink::new("See the plugins", "#plugins"),
                    NavLink::new("Read the Gantry docs", "/gantry/"),
                ],
            }

            section { id: "plugins", class: "center", "aria-labelledby": "plugins-heading",
                div { class: "pa-prose",
                    h2 { id: "plugins-heading", "Plugins" }
                    div { class: "grid",
                        // A released plugin's card leads to its documentation; an announced one
                        // says so.
                        for product in PRODUCTS.iter() {
                            match product.availability {
                                Availability::Available { docs } => rsx! {
                                    Card {
                                        title: product.name,
                                        text: product.summary,
                                        href: docs.to_owned(),
                                    }
                                },
                                Availability::ComingSoon => rsx! {
                                    Card {
                                        title: product.name,
                                        text: product.summary,
                                        badge: "coming soon".to_owned(),
                                    }
                                },
                            }
                        }
                    }
                }
            }

            section { id: "about", class: "center", "data-max": "md", "aria-labelledby": "about-heading",
                div { class: "pa-prose",
                    h2 { id: "about-heading", "About" }
                    p {
                        "Private Asylum is an independent studio building tools for Unreal \
                         Engine. Every plugin supports the three latest engine releases, and its \
                         reference documentation is generated from the code it ships."
                    }
                }
            }
        }
    })
}
