// Scribbles Rendr - converts Markdown files to a HTML+CSS website
// Copyright (C) 2023  Mateusz Czapliński
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use glob::glob;
use log::{error, info, warn};
use std::path::{Path, PathBuf};

mod logging;
mod pathinfo;

use pathinfo::PathInfo;

const BASE_SOURCES: &str = "*.md";
const DRAFT_SOURCES: &str = "_drafts/*.md";
const OUT_DIR: &str = "_html";

fn main() {
    logging::init_info();
    info!("👋😃");

    info!("Scanning {BASE_SOURCES} & {DRAFT_SOURCES}");
    let paths: Vec<_> = [BASE_SOURCES, DRAFT_SOURCES]
        .iter()
        .flat_map(|s| glob(s).unwrap())
        .filter_map(|result| match result {
            Err(e) => {
                warn!("{e}");
                None
            }
            Ok(path) => Some(path),
        })
        .filter_map(|path| match PathInfo::parse(&path) {
            Ok(info) => Some((path, info)),
            Err(err) => {
                warn!("Could not parse {path:?}: {err}");
                None
            }
        })
        .collect();
    println!("{paths:?}");

    // TODO: render articles to _html/
    // FIXME: purge targed dir before writing
    if let Err(err) = std::fs::create_dir_all(OUT_DIR) {
        error!("Could not create directory {OUT_DIR}: {err}");
    }
    for (path, info) in paths {
        if let Err(err) = md_to_html(&path, &info) {
            error!("Could not write {path:?}: {err}");
        }
    }

    // TODO: render list to _html/
    // TODO[LATER]: handle images
}

fn md_to_html(source_path: &Path, info: &PathInfo) -> anyhow::Result<()> {
    let markdown = std::fs::read_to_string(source_path)?;

    let parser = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(parser);
    markdown_it::plugins::extra::add(parser);
    markdown_it_footnote::add(parser);
    let ast = parser.parse(&markdown);
    let html = ast.render();
    // FIXME: add header & footer html from template
    // FIXME: fix relative links - strip .md etc.
    // TODO: copy images, css
    let mut destination: PathBuf = [OUT_DIR, &info.slug].iter().collect();
    destination.set_extension("html");
    info!("Writing {destination:?}.");
    std::fs::write(destination, html)?;
    Ok(())
}
