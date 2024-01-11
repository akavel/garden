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

//! Plugin for [markdown_it]. Finds fenced code blocks of type
//! `pikchr:render`, and replaces them with images containing their
//! content rendered via [pikchr].

// TODO[LATER]: how to make it trigger on some user-configured fence keyword?
const FENCE_KEYWORD: &str = "pikchr:render";
// const FENCE_KEYWORD: &str = "pikchr";
const CLASS: &str = "pikchr";

use markdown_it::parser::core::CoreRule;
use markdown_it::plugins::cmark::block::fence::CodeFence;
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};
use pikchr::Pikchr;

pub fn add(md: &mut MarkdownIt) {
    md.add_rule::<PikchrRule>();
}

pub struct PikchrRule;

impl CoreRule for PikchrRule {
    fn run(root: &mut Node, _md: &MarkdownIt) {
        root.walk_mut(|node, _| {
            let Some(fence) = node.cast::<CodeFence>() else {
                return;
            };
            // let info = &fence.info;
            // let pfx = &fence.lang_prefix;
            // println!("  fence.language={info} .pfx={pfx}");
            // TODO: && !fence.info.starts_with(FENCE_KEYWORD + " ")
            if &fence.info != FENCE_KEYWORD {
                return;
            }
            // let info = &fence.info;
            // println!("  fence.language={info}");
            let flags = Default::default();
            // TODO[LATER]: how to handle errors instead of .unwrap()?
            // TODO: add fence.info.strip_prefix(FENCE_KEYWORD) as class for CSS styling purposes
            // FIXME: CLASS seems not added to SVG - bug in library?
            let render = Pikchr::render(&fence.content, Some(CLASS), flags).unwrap();
            let svg = render.rendered().to_string();
            node.replace(PikchrSvg { svg });
        });
    }
}

#[derive(Debug)]
pub struct PikchrSvg {
    svg: String,
}

impl NodeValue for PikchrSvg {
    fn render(&self, _: &Node, fmt: &mut dyn Renderer) {
        fmt.text_raw(&self.svg);
    }
}
