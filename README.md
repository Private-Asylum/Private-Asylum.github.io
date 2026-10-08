# Private-Asylum.github.io

The Private Asylum hub site, served at **https://privateasylum.com/**.
Dioxus components rendered to static HTML at build time (no WASM, no
hydration), styled by [Yeti](https://github.com/foundation/yeti) and plain
modern CSS.

```bash
cargo run -p pvas-render --release              # renders into public/
cargo run -p pvas-render --release -- serve     # renders, then previews at http://127.0.0.1:8080/
```

The preview resolves folders to `index.html` and serves `404.html` for
anything missing, as Pages does. Product docs (`/gantry/`) live in their own
repositories, so they 404 locally.

## Layout

```
Crates/site/       pvas-site    this site's pages and content
Crates/render/     pvas-render  the binary: renders every page, hands the site to pvas-web-site
styles/            theme.css (Yeti tokens), site.css (this site's own rules)
```

Everything shared lives in
[pvas-web-shared](https://github.com/Private-Asylum/pvas-web-shared): the
components (document, nav, footer), the embedded Yeti build, writing the site
and the `build`/`serve` command line. It is pinned by commit in `Cargo.toml`;
moving the pin is how a change there reaches this site.

Adding a page: a module under `Crates/site/`, listed in `pages()` in
`site.rs`. Crates follow the house layout: no `src/`, the crate root beside
its `Cargo.toml`, and every module in `name/name.rs` wired with `#[path]`.

Full strict: clippy's `all`, `pedantic`, `nursery` and `cargo` groups are
denied workspace-wide, with a set of `restriction` lints and strict rustc and
rustdoc lints (`[workspace.lints]` in `Cargo.toml`, which also records the two
deliberate exceptions and why). CI runs `cargo fmt --check`, clippy with
`-D warnings` and `cargo doc` with `-D warnings` before it renders, so nothing
that breaks the policy deploys. Exceptions in code use `#[expect(..., reason
= "...")]`; `#[allow]` is itself denied.

## Styling

Yeti comes embedded from pvas-web-shared, which records the Yeti commit it was
built from. Style through Yeti's tokens in `styles/theme.css` first;
`styles/site.css` is unlayered, so it wins over Yeti's cascade layers without
specificity fights.

## Hosting

GitHub serves an organization site only from a repo named exactly
`<org>.github.io`: do not rename. This repo owns the custom domain; product
documentation repos are project sites that inherit it by path, and **the repo
name is the URL path**:

| Repo | URL |
| --- | --- |
| `Private-Asylum.github.io` | `https://privateasylum.com/` |
| `gantry` | `https://privateasylum.com/gantry/` |
| `terravoxel` | `https://privateasylum.com/terravoxel/` |

So those paths are reserved here: never create a page at a product's name.
Renaming a product repo breaks its URL (GitHub does not redirect project
sites). Pages source must be **GitHub Actions**; the workflow builds with
the toolchain in `rust-toolchain.toml`.

DNS (Route 53): apex A `185.199.108.153`, `.109.153`, `.110.153`, `.111.153`;
AAAA `2606:50c0:8000::153` through `8003::153`; `www` CNAME
`private-asylum.github.io`. The domain is verified at the org level, which
also protects its immediate subdomains. No wildcard records.
