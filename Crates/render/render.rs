//! Renders privateasylum.com into the folder GitHub Pages publishes, and previews it.
//!
//!     cargo run -p pvas-render --release -- [build] [--out <dir>]
//!     cargo run -p pvas-render --release -- serve [--out <dir>] [--port <port>]
//!
//! `serve` renders first, then serves the result on 127.0.0.1 (default port 8080).
//!
//! Writes every page from `pvas_site::pages()`, then copies in what the pages link to: the
//! vendored Yeti build, the site's stylesheets and static assets, `CNAME` and `.nojekyll`.
//! The output folder is replaced, never merged, so a removed page cannot linger. Defaults to
//! `public/` at the workspace root.

#![expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "a command-line tool reports its result and its usage to the terminal"
)]

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[path = "serve/serve.rs"]
mod serve;

/// Files from the Yeti build a page can link; its docs and editor files stay behind.
const YETI_FILES: &[&str] = &[
    "yeti.css",
    "yeti.min.css",
    "yeti.js",
    "yeti.min.js",
    "yeti.min.js.map",
    "LICENSE",
    "VENDORED",
];

/// Folders from the Yeti build a page can link.
const YETI_DIRS: &[&str] = &["js", "themes"];

/// What one run produced, for the closing report.
#[derive(Debug, Default)]
struct Summary {
    /// Pages rendered.
    pages: usize,
    /// Files copied: Yeti, stylesheets, assets and the root files.
    files: usize,
}

/// The workspace root, two levels above this crate's manifest.
fn workspace_root() -> io::Result<PathBuf> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
}

/// Copies one file, creating its folder. The byte count `fs::copy` reports is not needed.
fn copy_file(from: &Path, to: &Path) -> io::Result<()> {
    if let Some(parent) = to.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(from, to).map(drop)
}

/// Copies a folder recursively and returns how many files it copied.
fn copy_dir(from: &Path, to: &Path) -> io::Result<usize> {
    let mut count = 0;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            count += copy_dir(&entry.path(), &target)?;
        } else {
            copy_file(&entry.path(), &target)?;
            count += 1;
        }
    }
    Ok(count)
}

/// Renders the site into `out`, replacing whatever was there.
fn run(out: &Path) -> io::Result<Summary> {
    let root = workspace_root()?;
    let mut summary = Summary::default();

    if out.exists() {
        fs::remove_dir_all(out)?;
    }
    fs::create_dir_all(out)?;

    for page in pvas_site::pages() {
        let html = pvas_web::render_document(&page.meta, (page.body)());
        let path = out.join(page.file);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, html)?;
        summary.pages += 1;
    }

    let yeti_from = root.join("vendor/yeti");
    let yeti_to = out.join("vendor/yeti");
    for file in YETI_FILES {
        copy_file(&yeti_from.join(file), &yeti_to.join(file))?;
        summary.files += 1;
    }
    for dir in YETI_DIRS {
        summary.files += copy_dir(&yeti_from.join(dir), &yeti_to.join(dir))?;
    }

    summary.files += copy_dir(&root.join("styles"), &out.join("css"))?;
    if root.join("assets").is_dir() {
        summary.files += copy_dir(&root.join("assets"), out)?;
    }

    copy_file(&root.join("CNAME"), &out.join("CNAME"))?;
    fs::write(out.join(".nojekyll"), "")?;
    summary.files += 2;

    Ok(summary)
}

/// The usage line, printed for any argument this tool does not understand.
const USAGE: &str = "usage: pvas-render [build] [--out <dir>]\n       pvas-render serve [--out <dir>] [--port <port>]";

/// The preview server's default port.
const DEFAULT_PORT: u16 = 8080;

/// What a run was asked to do.
#[derive(Debug, PartialEq, Eq)]
struct Command {
    /// Serve the result after rendering it.
    serve: bool,
    /// The output folder; the workspace's `public/` when not given.
    out: Option<PathBuf>,
    /// The preview server's port.
    port: u16,
}

/// Parses `[build|serve] [--out <dir>] [--port <port>]`. `--port` only applies to `serve`.
fn parse_args(args: impl Iterator<Item = String>) -> Result<Command, &'static str> {
    let mut args = args.peekable();
    let serve = match args.peek().map(String::as_str) {
        Some("serve") => true,
        Some("build") => false,
        _ => {
            return parse_options(args, false);
        }
    };
    let _subcommand = args.next();
    parse_options(args, serve)
}

/// Parses the options after the subcommand.
fn parse_options(
    mut args: impl Iterator<Item = String>,
    serve: bool,
) -> Result<Command, &'static str> {
    let mut command = Command {
        serve,
        out: None,
        port: DEFAULT_PORT,
    };
    while let Some(arg) = args.next() {
        match (arg.as_str(), args.next()) {
            ("--out", Some(dir)) => command.out = Some(PathBuf::from(dir)),
            ("--port", Some(port)) if serve => command.port = port.parse().map_err(|_| USAGE)?,
            _ => return Err(USAGE),
        }
    }
    Ok(command)
}

fn main() -> ExitCode {
    let command = match parse_args(env::args().skip(1)) {
        Ok(command) => command,
        Err(usage) => {
            eprintln!("{usage}");
            return ExitCode::from(2);
        }
    };

    let out = match command
        .out
        .map_or_else(|| workspace_root().map(|root| root.join("public")), Ok)
    {
        Ok(out) => out,
        Err(error) => {
            eprintln!("error: cannot locate the workspace root: {error}");
            return ExitCode::FAILURE;
        }
    };

    match run(&out) {
        Ok(summary) => println!(
            "Rendered {} page(s) and copied {} file(s) into {}",
            summary.pages,
            summary.files,
            out.display()
        ),
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
    }

    if command.serve
        && let Err(error) = serve::serve(&out, command.port)
    {
        eprintln!("error: {error}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
