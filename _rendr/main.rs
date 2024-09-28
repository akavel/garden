// Garden Rendr - converts Markdown files to a HTML+CSS website
// Copyright (C) 2023-2024  Mateusz Czapliński
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
use std::path::PathBuf;

mod html;
mod logging;
mod md_pikchr;
mod pathinfo;

use html::Html;
use pathinfo::PathInfo;

const SOURCES: &str = "*.md:@seed/*.md:@snip/*.md";
const RAW: &str =
    "favicon.ico:*.pdf:@seed/*.pdf:*.png:@seed/*.png:*.jpg:@seed/*.jpg:*.svg:@seed/*.svg";
const SCRIPT_PATH: &str = "_bloat/bloat.lua";
const OUT_DIR: &str = "_html.out";

fn main() -> anyhow::Result<()> {
    logging::init_info();
    info!("Hi! 👋😃");

    info!("Scanning {SOURCES}");
    let paths: Vec<_> = SOURCES
        .split(':')
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

    info!("Copying {RAW}");
    let raw_files =
        RAW.split(':')
            .flat_map(|s| glob(s).unwrap())
            .filter_map(|result| match result {
                Err(e) => {
                    warn!("{e}");
                    None
                }
                Ok(path) => Some(path),
            });
    let out_dir = PathBuf::from(OUT_DIR);
    for f in raw_files {
        let Some(name) = f.file_name() else {
            continue;
        };
        println!("{f:?} -> {out_dir:?}/{name:?}");
        std::fs::copy(&f, out_dir.join(name)).unwrap();
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
    md_pikchr::add(parser); // TODO: here or earlier?
    markdown_it::plugins::extra::add(parser);
    markdown_it_footnote::add(parser);
    markdown_it_sub::add(parser);
    markdown_it_sup::add(parser);
    let ast = parser.parse(markdown);
    // ast.walk(|node, _| {
    //     let name = node.name();
    //     let skip = &[
    //         "::Paragraph",
    //         "::Text",
    //         "::Em",
    //         "::backticks::CodeInline",
    //         "::Link",
    //     ];
    //     for s in skip {
    //         if name.ends_with(s) { return; }
    //     }
    //     println!("  {name:?}");
    //     use markdown_it::plugins::cmark::block::fence;
    //     let Some(fence) = node.cast::<fence::CodeFence>() else { return };
    //     let info = &fence.info;
    //     let pfx = &fence.lang_prefix;
    //     println!("<fence> info={info:?} lang_prefix={pfx:?}");
    // });
    let html = ast.render();
    RawHtml::parse_fragment(&html)
}
