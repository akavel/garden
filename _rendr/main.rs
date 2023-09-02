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

use ego_tree::NodeId;
use glob::glob;
use log::{error, info, warn};
use mlua::prelude::*;
use scraper::{Html, Selector};

mod logging;
mod pathinfo;

use pathinfo::PathInfo;

const BASE_SOURCES: &str = "*.md";
const DRAFT_SOURCES: &str = "_drafts/*.md";
const SCRIPT_PATH: &str = "_bloat/bloat.lua";
const OUT_DIR: &str = "_html.out";

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

    // FIXME: purge target dir before writing
    if let Err(err) = std::fs::create_dir_all(OUT_DIR) {
        error!("Could not create directory {OUT_DIR}: {err}");
    }

    let lua = Lua::new();

    // Expose limited HTML parser & DOM functionality to Lua
    let html_mod = lua.create_table().unwrap();
    let parse = lua
        .create_function(|_, (text,): (String,)| Ok(Htmler::from(Html::parse_document(&text))))
        .unwrap();
    html_mod.set("parse", parse).unwrap();
    let from_md = lua
        .create_function(|_, (text,): (String,)| Ok(Htmler::from(md_to_html(&text))))
        .unwrap();
    html_mod.set("from_md", from_md).unwrap();
    lua.globals().set("html", html_mod).unwrap();

    // TODO: render list to _html/
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

    let script = std::fs::read_to_string(SCRIPT_PATH).unwrap();
    lua.load(script).set_name(SCRIPT_PATH).exec().unwrap();

    // TODO[LATER]: handle images
}

#[derive(Clone)]
struct Htmler {
    html: scraper::Html,
}

impl From<scraper::Html> for Htmler {
    fn from(html: scraper::Html) -> Self {
        Self { html }
    }
}

impl mlua::UserData for Htmler {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("to_string", |_, htmler, ()| {
            Ok(htmler.html.html())
        });

        methods.add_method("clone", |_, htmler, ()| {
            Ok(Htmler::from(htmler.html.clone()))
        });

        methods.add_method("find", |_, htmler, (selector,): (String,)| {
            let maybe_id = node_id_by_selector(&htmler.html, &selector);
            Ok(maybe_id.map(NodeIdWrap))
        });

        methods.add_method("root", |_, htmler, ()| {
            Ok(NodeIdWrap(htmler.html.tree.root().id()))
        });

        methods.add_method_mut("delete_children", |_, htmler, (id,): (NodeIdWrap,)| {
            delete_children(&mut htmler.html, id.0);
            Ok(())
        });

        methods.add_method_mut("add_children", |_, htmler, args: LuaMultiValue| {
            let dst_node = *borrow_ud::<NodeIdWrap>(args.get(0)).unwrap();
            let src = borrow_ud::<Htmler>(args.get(1)).unwrap();
            let src_node = *borrow_ud::<NodeIdWrap>(args.get(2)).unwrap();
            add_children(&mut htmler.html, dst_node.0, &src.html, src_node.0);
            Ok(())
        });
    }
}

fn borrow_ud<'a, T: 'static>(v: Option<&'a LuaValue<'a>>) -> Option<std::cell::Ref<'a, T>> {
    v.and_then(|v| v.as_userdata().and_then(|ud| ud.borrow::<T>().ok()))
}

#[derive(Copy, Clone, Debug)]
struct NodeIdWrap(NodeId);

impl mlua::UserData for NodeIdWrap {}

impl<'lua> mlua::FromLua<'lua> for NodeIdWrap {
    fn from_lua(value: mlua::Value<'lua>, _: &'lua Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::UserData(ud) => Ok(*ud.borrow::<Self>()?),
            _ => unreachable!(),
        }
    }
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

fn node_id_by_selector(html: &Html, selector: &str) -> Option<NodeId> {
    let selector = Selector::parse(selector).ok()?;
    html.select(&selector).next().map(|n| n.id())
}

fn delete_children(html: &mut Html, parent_id: NodeId) {
    // Note: per https://github.com/causal-agent/scraper/issues/125, it seems
    // we cannot delete nodes from a tree while iterating over it.
    let node_ref = html.tree.get(parent_id).unwrap(); // FIXME: unwrap
    let children = node_ref.children().map(|n| n.id()).collect::<Vec<_>>();
    for child in children {
        html.tree.get_mut(child).unwrap().detach(); // FIXME: unwrap
    }
}

fn add_children(dst: &mut Html, dst_id: NodeId, src: &Html, src_id: NodeId) {
    // TODO[LATER]: arrrrgh, it looks so complex and inefficient; is there simpler way?
    use std::collections::VecDeque;
    let mut queue = VecDeque::<(NodeId, NodeId)>::new();
    for child in src.tree.get(src_id).iter().flat_map(|n| n.children()) {
        queue.push_back((dst_id, child.id()));
    }
    loop {
        let Some((dst_id, src_id)) = queue.pop_front() else {
            break;
        };
        let mut dst_node = dst.tree.get_mut(dst_id).unwrap(); // FIXME: unwrap
        let src_node_ref = src.tree.get(src_id).unwrap(); // FIXME: unwrap
        let src_node = src_node_ref.value();
        // HACK
        let name_is_html = |e: &&scraper::node::Element| e.name() == "html";
        let not_html_node = src_node.as_element().filter(name_is_html).is_none();
        let new_id = if not_html_node {
            dst_node.append(src_node.clone()).id()
        } else {
            dst_id
        };
        for child in src_node_ref.children() {
            queue.push_back((new_id, child.id()));
        }
    }
}
