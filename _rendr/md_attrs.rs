// This file is part of Garden Rendr.
// Copyright (C) 2023-2024  Mateusz Czapliński
//
// Garden Rendr is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// Garden Rendr is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Plugin for [markdown_it]. Finds pandoc-style attributes like:
//! `{#id .class1 .class2 key=value key2=value2}`
//! and adds them to the preceding node.

use markdown_it::parser::core::CoreRule;
// use markdown_it::parser::extset::{MarkdownItExt, NodeExt};
// use markdown_it::parser::inline::builtin::InlineParserRule;
use markdown_it::parser::inline::{InlineRule, Text};
use markdown_it::{MarkdownIt, Node, NodeValue};

pub fn add(md: &mut MarkdownIt) {
    use markdown_it::parser::inline::builtin::InlineParserRule;
    md.inline.add_rule::<AttrsScanner>().after_all();
    // md.add_rule::<AttrsScanner>().before::<AttrsApplier>();
    md.add_rule::<AttrsApplier>().after::<InlineParserRule>().before::<crate::md_gallery_row::GalleryRowRule>();
}

pub struct AttrsScanner;

impl InlineRule for AttrsScanner {
    const MARKER: char = '{';

    fn run(state: &mut markdown_it::parser::inline::InlineState) -> Option<(Node, usize)> {
        let input = &state.src[state.pos..state.pos_max];
        let (attrs, length) = attrs_pattern::attrs_with_length(input).ok()?;
        println!("PARSED: {attrs:?}");
        Some((
            Node::new(Attrs::new(attrs)),
            length,
        ))
    }
}

#[derive(Debug)]
pub struct Attrs {
    attrs: Vec<(String, String)>,
}

impl Attrs {
    fn new(attrs: Vec<(String, String)>) -> Self {
        Self { attrs }
    }
}

impl NodeValue for Attrs {
    fn render(&self, _node: &Node, _fmt: &mut dyn markdown_it::Renderer) { }
}

peg::parser! { grammar attrs_pattern() for str {
    #[no_eof]
    pub rule attrs_with_length() -> (Vec<(String, String)>, usize)
        = "{" attrs:(attr() ++ whitespace()) "}" len:position!() { (attrs, len) }

    // Helper sub-parsers.
    rule attr() -> (String, String)
        = attr:( id() / class() / key_value() ) { attr }
    rule id() -> (String, String)
        = "#" w:word() { ("id".to_string(), w) }
    rule class() -> (String, String)
        = "." w:word() { ("class".to_string(), w) }
    rule key_value() -> (String, String)
        // TODO: handle single & double quotes in value
        = k:word() "=" v:word() { (k, v) }

    // Lowest-level "words" parsers.
    rule word() -> String
        = slice:$( start_char() next_char()* ) { slice.to_string() }
        // = slice:$( ['a'..='z' | 'A'..='Z'] ['a'..='z' | 'A'..='Z' | '0'..='9']* ) { slice.to_string() }
    rule start_char() -> char
        = c:['a'..='z' | 'A'..='Z' | '-' | '_'] { c }
    rule next_char() -> char
        = c:(start_char() / ['0'..='9']) { c }
    rule whitespace() -> String
        = slice:$( [ ' ' | '\t' ]+ ) { slice.to_string() }
}}

pub struct AttrsApplier;

impl CoreRule for AttrsApplier {
    fn run(root: &mut Node, _: &MarkdownIt) {
        let mut last_attrs: Vec<(String, String)> = Vec::new();
        walk_reverse_mut(root, |node| {
            // If Attrs found, steal them into last_attrs temporary storage.
            if let Some(attrs) = node.cast_mut::<Attrs>() {
                println!("FOUND ATTRS: {attrs:?}");
                std::mem::swap(&mut last_attrs, &mut attrs.attrs);
                return;
            }
            // If we don't store anything in last_attrs, we don't have anything to add.
            if last_attrs.len() == 0 {
                return;
            }
            // don't apply attributes to text nodes
            if node.is::<Text>() {
                return;
            }
            // println!("APPLYING TO NODE: {node:?}");
            apply_attrs(node, &mut last_attrs);
        });
    }
}

fn walk_reverse_mut(node: &mut Node, mut f: impl FnMut(&mut Node)) {
    fn walk_recursive(node: &mut Node, f: &mut impl FnMut(&mut Node)) {
        for child in node.children.iter_mut().rev() {
            // stacker use and general shape copied from markdown_it's walk_post_mut
            stacker::maybe_grow(64*1024, 1024*1024, || {
                walk_recursive(child, f);
            });
        }
        f(node);
    }
    walk_recursive(node, &mut f);
}

fn apply_attrs(node: &mut Node, attrs: &mut Vec<(String, String)>) {
    // Unfortunately, node.attrs has 'static str as key, so we can only allow
    // previously-known key names :/
    // Also, we need to dedup multiple class names into one attr.
    const ID: &str = "id";
    const CLASS: &str = "class";
    const STYLE: &str = "style";
    let mut classes: Vec<String> = Vec::new();
    for (k, v) in attrs.drain(..) {
        match k.as_str() {
            ID => node.attrs.push((ID, v)),
            STYLE => node.attrs.push((STYLE, v)),
            CLASS => classes.push(v),
            _ => (), // TODO: warn "unsupported attribute $k" :/
        }
    }
    if classes.len() > 0 {
        node.attrs.push((CLASS, classes.join(" ")));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn image_classes() {
        let input = "![alt](url){.class1 .class2}";
        let mut md = markdown_it::MarkdownIt::new();
        markdown_it::plugins::cmark::add(&mut md);
        add(&mut md);
        let root = md.parse(input);
        let html = root.render();
        assert_eq!(html.trim(), r#"<p><img class="class1 class2" src="url" alt="alt"></p>"#);
    }

    #[test]
    fn header_id_after() {
        let input = "# header{#id}";
        let mut md = markdown_it::MarkdownIt::new();
        markdown_it::plugins::cmark::add(&mut md);
        add(&mut md);
        let root = md.parse(input);
        let html = root.render();
        assert_eq!(html.trim(), r#"<h1 id="id">header</h1>"#);
    }

    #[test]
    fn header_id_before() {
        let input = "# {#id}header";
        let mut md = markdown_it::MarkdownIt::new();
        markdown_it::plugins::cmark::add(&mut md);
        add(&mut md);
        let root = md.parse(input);
        let html = root.render();
        assert_eq!(html.trim(), r#"<h1 id="id">header</h1>"#);
    }
}
