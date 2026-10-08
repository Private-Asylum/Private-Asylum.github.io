//! The home page: who we are, what we make, where the documentation is.

use dioxus::prelude::*;

use crate::{Availability, PRODUCTS};
use crate::chrome::page;

/// The home page's `body`.
pub(crate) fn body() -> Element {
    page(rsx! {
        main { id: "content", class: "stack", "data-gap": "3xl",
            section { class: "center", "aria-labelledby": "headline",
                div { class: "hero", "data-threshold": "md", "data-gap": "xl", "data-height": "md",
                    div {
                        h1 { id: "headline", "Tools for building worlds in Unreal Engine." }
                        p { class: "lede",
                            "Private Asylum makes Unreal Engine plugins for content pipelines, \
                             planet-scale terrain and star systems, documented from the source \
                             they ship."
                        }
                        div { class: "cluster", "data-gap": "sm",
                            a { class: "button", href: "#plugins", "See the plugins" }
                            a { class: "button", href: "/gantry/", "data-emphasis": "medium",
                                "Read the Gantry docs"
                            }
                        }
                    }
                }
            }

            section { id: "plugins", class: "center", "aria-labelledby": "plugins-heading",
                div { class: "stack", "data-gap": "lg",
                    h2 { id: "plugins-heading", "Plugins" }
                    div { class: "grid",
                        for product in PRODUCTS.iter() {
                            article { class: "card",
                                h3 { "{product.name}" }
                                p { "{product.summary}" }
                                match product.availability {
                                    Availability::Available { docs } => rsx! {
                                        a { href: "{docs}", "Documentation →" }
                                    },
                                    Availability::ComingSoon => rsx! {
                                        p { class: "pa-pending", "Coming soon." }
                                    },
                                }
                            }
                        }
                    }
                }
            }

            section { id: "about", class: "center", "data-max": "md", "aria-labelledby": "about-heading",
                h2 { id: "about-heading", "About" }
                p {
                    "Private Asylum is an independent studio building tools for Unreal \
                     Engine. Every plugin supports the three latest engine releases, and its \
                     reference documentation is generated from the code it ships."
                }
            }
        }
    })
}
