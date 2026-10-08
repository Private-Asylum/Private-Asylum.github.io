//! The bar and footer as this site configures them.

use dioxus::prelude::*;

use pvas_web::{Footer, Nav, NavLink};

/// Wraps a page's `main` in the site's `body`: skip link, bar, the page, footer.
pub(crate) fn page(main: Element) -> Element {
    rsx! {
        body { class: "shell", "data-gap": "xl",
            a { href: "#content", "Skip to content" }
            Nav {
                brand: NavLink::new(crate::SITE_NAME, "/"),
                links: vec![
                    NavLink::new("Plugins", "/#plugins"),
                    NavLink::new("Gantry docs", "/gantry/"),
                    NavLink::new("About", "/#about"),
                ],
                action: Some(NavLink::new("GitHub", "https://github.com/Private-Asylum")),
            }
            {main}
            Footer {
                copyright: "© 2026 Private Asylum LLC".to_owned(),
                links: vec![NavLink::new("GitHub", "https://github.com/Private-Asylum")],
            }
        }
    }
}
