// Garden Rendr - converts Markdown files to a HTML+CSS website
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
use mlua::prelude::*;
use scraper::Html as RawHtml;

mod html;
mod logging;
mod pathinfo;

use html::Html;
use pathinfo::PathInfo;

const BASE_SOURCES: &str = "*.md";
const DRAFT_SOURCES: &str = "@seed/*.md";
const SCRIPT_PATH: &str = "_bloat/bloat.lua";
const OUT_DIR: &str = "_html.out";

fn main() -> anyhow::Result<()> {
    logging::init_info();
    info!("Hi! 👋😃");

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

    // FIXME: purge target dir before writing
    if let Err(err) = std::fs::create_dir_all(OUT_DIR) {
        error!("Could not create directory {OUT_DIR}: {err}");
    }

    let lua = Lua::new();

    // Expose limited HTML parser & DOM functionality to Lua
    let html_mod = lua.create_table().unwrap();
    let parse = lua
        .create_function(|_, (text,): (String,)| Ok(Html::from(RawHtml::parse_document(&text))))
        .unwrap();
    html_mod.set("parse", parse).unwrap();
    let from_md = lua
        .create_function(|_, (text,): (String,)| Ok(Html::from(md_to_html(&text))))
        .unwrap();
    html_mod.set("from_md", from_md).unwrap();
    let new = lua
        .create_function(|_, ()| Ok(Html::from(RawHtml::new_document())))
        .unwrap();
    html_mod.set("new", new).unwrap();
    lua.globals().set("html", html_mod).unwrap();

    // Pass articles metadata to Lua
    let articles = lua.create_table().unwrap();
    for (path, info) in paths {
        let tags = lua.create_sequence_from(info.tags).unwrap();
        let article = lua.create_table().unwrap();
        article.set("src", path.to_str()).unwrap();
        article.set("slug", info.slug).unwrap();
        article.set("datetime", info.datetime).unwrap();
        article.set("extension", info.extension).unwrap();
        article.set("tags", tags).unwrap();
        articles.push(article).unwrap();
    }
    lua.globals().set("articles", articles).unwrap();

    // Run Lua script intended to process the articles
    let script = std::fs::read_to_string(SCRIPT_PATH).unwrap();
    lua.load(script).set_name(SCRIPT_PATH).exec()?;

    // TODO[LATER]: handle images

    Ok(())
}

fn md_to_html(markdown: &str) -> RawHtml {
    let parser = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(parser);
    markdown_it::plugins::extra::add(parser);
    markdown_it_footnote::add(parser);
    let ast = parser.parse(markdown);
    let html = ast.render();
    RawHtml::parse_fragment(&html)
}
