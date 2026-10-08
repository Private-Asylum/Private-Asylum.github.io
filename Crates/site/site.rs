//! privateasylum.com: every page, as data the renderer walks.
//!
//! A page is a path, its head metadata and a function that builds its `body`. Adding a page means
//! adding a module and listing it in [`pages`]; there is no router, because nothing routes at run
//! time. Product documentation does not live here: each product is its own repository, served
//! under its own path on this domain (`/gantry/`), so those paths are reserved.

use dioxus::prelude::*;

use pvas_web_shared::assets::HOUSE_STYLESHEETS;
use pvas_web_shared::components::PageMeta;

#[path = "chrome/chrome.rs"]
mod chrome;
#[path = "home/home.rs"]
mod home;
#[path = "not_found/not_found.rs"]
mod not_found;
#[path = "products/products.rs"]
mod products;

pub use products::{Availability, PRODUCTS, Product};

/// The site's canonical origin, without a trailing slash.
pub const ORIGIN: &str = "https://privateasylum.com";

/// The site's name, used in every title.
pub const SITE_NAME: &str = "Private Asylum";

/// One page of the site.
#[derive(Debug)]
pub struct Page {
    /// Where the page is written, relative to the output root: `index.html`, `404.html`.
    pub file: &'static str,
    /// The page's metadata.
    pub meta: PageMeta,
    /// Builds the page's `body` element.
    pub body: fn() -> Element,
}

/// Stylesheets every page loads, in order: the house (Yeti, then the house style), then the
/// site's theme, which sets its hues.
fn stylesheets() -> Vec<String> {
    let mut sheets: Vec<String> = HOUSE_STYLESHEETS
        .iter()
        .map(|sheet| format!("/{sheet}"))
        .collect();
    sheets.push("/css/theme.css".to_owned());
    sheets
}

/// Head metadata for a page: its title with the site name, and its canonical URL when it has
/// one. `path` is site-absolute, e.g. `/`.
fn meta(title: &str, description: &str, path: Option<&str>) -> PageMeta {
    PageMeta {
        title: if title.is_empty() {
            SITE_NAME.to_owned()
        } else {
            format!("{title} · {SITE_NAME}")
        },
        description: description.to_owned(),
        canonical: path.map(|p| format!("{ORIGIN}{p}")).unwrap_or_default(),
        stylesheets: stylesheets(),
    }
}

/// Every page the site publishes.
#[must_use]
pub fn pages() -> Vec<Page> {
    vec![
        Page {
            file: "index.html",
            meta: meta(
                "",
                "Private Asylum builds Unreal Engine plugins for content pipelines, planet-scale \
                 terrain and star systems.",
                Some("/"),
            ),
            body: home::body,
        },
        Page {
            file: "404.html",
            meta: meta("Not found", "This page does not exist.", None),
            body: not_found::body,
        },
    ]
}
