// This file is part of Scribbles Rendr.
// Copyright (C) 2023  Mateusz Czapliński
//
// Scribbles Rendr is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// Scribbles Rendr is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! The module contains functions for parsing and editing HTML trees.

use ego_tree::NodeId;
use mlua::prelude::*;
use scraper::{Html as RawHtml, Selector};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Html {
    raw: Rc<RefCell<RawHtml>>,
    node_id: Option<NodeId>,
}

impl Html {
    fn view(&self) -> Self {
        Self {
            raw: Rc::clone(&self.raw),
            node_id: self.node_id,
        }
    }

    fn view_with_id(&self, id: NodeId) -> Self {
        Self {
            raw: Rc::clone(&self.raw),
            node_id: Some(id),
        }
    }

    fn id_or_root(&self) -> NodeId {
        self.node_id
            .unwrap_or_else(|| self.raw.borrow().tree.root().id())
    }
}

impl From<RawHtml> for Html {
    fn from(raw: RawHtml) -> Self {
        Self {
            raw: Rc::new(RefCell::new(raw)),
            node_id: None,
        }
    }
}

impl<'lua> mlua::FromLua<'lua> for Html {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> mlua::Result<Self> {
        match value {
            LuaValue::UserData(ud) => Ok(ud.borrow::<Self>()?.view()),
            _ => Err(LuaError::RuntimeError(format!(
                "expected Html userdata, got: {}",
                value.type_name(),
            ))),
        }
    }
}

impl mlua::UserData for Html {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("to_string", |_, html, ()| Ok(html.raw.borrow().html()));

        methods.add_method("clone", |_, html, ()| {
            Ok(Html::from(html.raw.borrow().clone()))
        });

        methods.add_method("find", |_, html, (selector,): (String,)| {
            let maybe_id = node_id_by_selector(&html.raw.borrow(), &selector);
            let maybe_html = maybe_id.map(|id| html.view_with_id(id));
            Ok(maybe_html)
        });

        methods.add_method("root", |_, html, ()| {
            let id = html.raw.borrow().tree.root().id();
            Ok(html.view_with_id(id))
        });

        methods.add_method_mut("delete_children", |_, html, ()| {
            let id = html.id_or_root();
            delete_children(&mut html.raw.borrow_mut(), id);
            Ok(())
        });

        methods.add_method_mut("add_children", |_, html, (src,): (Html,)| {
            let dst_id = html.id_or_root();
            let src_id = src.id_or_root();
            add_children(
                &mut html.raw.borrow_mut(),
                dst_id,
                &src.raw.borrow(),
                src_id,
            );
            Ok(())
        });

        methods.add_method_mut("add_text", |_, html, (s,): (String,)| {
            let id = html.id_or_root();
            let mut raw = html.raw.borrow_mut();
            let mut dst = raw.tree.get_mut(id).unwrap(); // FIXME: unwrap
            let text = scraper::node::Text { text: s.into() };
            dst.append(scraper::Node::Text(text));
            Ok(())
        });

        methods.add_method_mut("set_attr", |_, html, (k, v): (String, String)| {
            let id = html.id_or_root();
            let mut raw = html.raw.borrow_mut();
            let mut dst = raw.tree.get_mut(id).unwrap(); // FIXME: unwrap
            use scraper::Node;
            if let Node::Element(el) = dst.value() {
                use markup5ever::{LocalName, Namespace, QualName};
                let attr = QualName::new(None, Namespace::from(""), LocalName::from(k));
                el.attrs.insert(attr, v.into());
            }
            Ok(())
        });

        // TODO: get_text(id) -> String  // concatenated from whole subtree
        // TODO: get_attr(id, String) -> String
    }
}

fn node_id_by_selector(html: &RawHtml, selector: &str) -> Option<NodeId> {
    let selector = Selector::parse(selector).ok()?;
    html.select(&selector).next().map(|n| n.id())
}

fn delete_children(html: &mut RawHtml, parent_id: NodeId) {
    // Note: per https://github.com/causal-agent/scraper/issues/125, it seems
    // we cannot delete nodes from a tree while iterating over it.
    let node_ref = html.tree.get(parent_id).unwrap(); // FIXME: unwrap
    let children = node_ref.children().map(|n| n.id()).collect::<Vec<_>>();
    for child in children {
        html.tree.get_mut(child).unwrap().detach(); // FIXME: unwrap
    }
}

fn add_children(dst: &mut RawHtml, dst_id: NodeId, src: &RawHtml, src_id: NodeId) {
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
