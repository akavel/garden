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
use scraper::{Html, Selector};
use std::path::{Path, PathBuf};

mod logging;
mod pathinfo;

use pathinfo::PathInfo;

const BASE_SOURCES: &str = "*.md";
const DRAFT_SOURCES: &str = "_drafts/*.md";
const HTML_TEMPLATE: &str = "_bloat/bloat.html";
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

    // FIXME: purge target dir before writing
    if let Err(err) = std::fs::create_dir_all(OUT_DIR) {
        error!("Could not create directory {OUT_DIR}: {err}");
    }
    for (path, info) in paths {
        if let Err(err) = make_html(&path, &info) {
            error!("Could not write {path:?}: {err}");
        }
    }

    // TODO: render list to _html/
    // TODO[LATER]: handle images
}

fn make_html(source_path: &Path, info: &PathInfo) -> anyhow::Result<()> {
    use std::fs;

    let markdown = fs::read_to_string(source_path)?;
    let html_fragment = md_to_html(&markdown);
    // FIXME[optimization]: reuse, don't load every time anew
    // TODO[LATER]: configurable HTML_TEMPLATE
    // TODO[LATER]: different templates for different files; don't really need yet
    let mut html_template = Html::parse_document(&fs::read_to_string(HTML_TEMPLATE)?);

    // Build the result. Idea of working on HTML blatantly stolen from Soupault.app <3
    // TODO[LATER]: move this either to Lua, or .ini, or something akin
    // FIXME: remove unwrap
    let selector = Selector::parse("#content").unwrap();
    let mut selection = html_template.select(&selector);
    let node_id = selection.next().unwrap().id(); // FIXME: unwrap
    replace_children(&mut html_template, node_id, html_fragment);
    // for element in html_template.select(&selector) {
    //     replace_children(element, html_fragment);
    // }

    // FIXME: add header & footer html from template
    // FIXME: extract H1 title from AST, put in <html><head><title>...</title>
    // FIXME: add <article> & <main> (via template) -> maybe replace <main> with rendered markdown
    // FIXME: fix relative links - strip .md etc.
    // TODO: copy images, css
    let mut destination: PathBuf = [OUT_DIR, &info.slug].iter().collect();
    destination.set_extension("html");
    info!("Writing {destination:?}.");
    let html = html_template;
    std::fs::write(destination, html.html())?;
    Ok(())
}

fn md_to_html(markdown: &str) -> scraper::Html {
    let parser = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(parser);
    markdown_it::plugins::extra::add(parser);
    markdown_it_footnote::add(parser);
    let ast = parser.parse(markdown);
    let html = ast.render();
    scraper::Html::parse_fragment(&html)
}

fn replace_children(html: &mut Html, node_id: ego_tree::NodeId, fragment: Html) {
    let node_ref = html.tree.get(node_id).unwrap(); // FIXME: unwrap
    let children = node_ref.children().map(|n| n.id()).collect::<Vec<_>>();
    for child in children {
        html.tree.get_mut(child).unwrap().detach(); // FIXME: unwrap
    }
}
