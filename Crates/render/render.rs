//! privateasylum.com's binary: renders every page and hands the site to `pvas-web-site`.
//!
//!     cargo run -p pvas-render --release                 # writes public/
//!     cargo run -p pvas-render --release -- serve        # writes it and previews it
//!
//! The command line, the output folder handling and the preview server all come from
//! `pvas_web_shared::site`; this file only says what the site is made of.

use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use pvas_web_shared::components::render_document;
use pvas_web_shared::site::{Site, SiteFile, run};

/// The workspace root, two levels above this crate's manifest.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Everything the site publishes: its pages, the house (Yeti and the house style), its own
/// stylesheets and its `CNAME`.
fn build() -> io::Result<Site> {
    let root = workspace_root();
    let mut site = Site::new().with_house();

    for page in pvas_site::pages() {
        let html = render_document(&page.meta, (page.body)());
        site.add(SiteFile::page(page.file, html));
    }

    site.extend(SiteFile::dir(&root.join("styles"), "css")?);
    site.add(SiteFile::page(
        "CNAME",
        std::fs::read_to_string(root.join("CNAME"))?,
    ));
    Ok(site)
}

fn main() -> ExitCode {
    run("pvas-render", &workspace_root().join("public"), build)
}
