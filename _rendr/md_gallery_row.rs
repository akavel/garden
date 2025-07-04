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

//! Plugin for [markdown_it]. Finds paragraphs comprised of only images
//! and converts them into a <div> block with given class and with
//! flex-grow style applied based on width & height listed for each image.

// gallery-1 first attempt:
// LATER: let's try this image syntax: ![<alt-text>](<url-no-spaces> width=123 height=456 "optional title")
// LATER: add support for figure+figcaption
// LATER: add support for alt-text


const CLASS: &str = "gallery-row";

use std::path::Path;

use markdown_it::parser::core::CoreRule;
use markdown_it::plugins::cmark::{
    inline::{image::Image, newline::Softbreak},
    block::paragraph::Paragraph,
};
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

pub fn add(md: &mut MarkdownIt) {
    use markdown_it::parser::inline::builtin::InlineParserRule;
    md.add_rule::<GalleryRowRule>().after::<InlineParserRule>();
}

pub struct GalleryRowRule;

impl CoreRule for GalleryRowRule {
    fn run(root: &mut Node, _md: &MarkdownIt) {
        // XXX UGLY HACK XXX
        let img_dir = Path::new(super::OUT_DIR);
        // XXX

        root.walk_mut(|node, _| {
            let Some(_para) = node.cast::<Paragraph>() else {
                return;
            };

            let mut images = Vec::new();
            // println!(" para");
            for child in &node.children {
                if let Some(_softbreak) = child.cast::<Softbreak>() {
                    continue;
                }
                // println!("  {:?}", child.node_type);
                let Some(image) = child.cast::<Image>() else {
                    return;
                };
                let path = img_dir.join(&image.url);
                let Ok(mut size) = imagesize::size(&path) else {
                    return;
                };

                // Try making a thumbnail of the image.
                // TODO(LATER): make the thumbnails after all images in the row are collected,
                // sizing them proportionally
                let mut thumb_url = image.url.clone();
                if let Ok(image_reader) = image::ImageReader::open(&path)
                    && let Ok(image_data) = image_reader.decode()
                {
                    const MAXW: u32 = 1000;
                    const MAXH: u32 = 1000;
                    println!(" (thumbnailing {})", &image.url);
                    let thumb = image_data.resize(MAXW, MAXH, image::imageops::FilterType::CatmullRom);
                    // TODO(LATER): handle subdirectory urls as well
                    let new_thumb_url = "thumb.".to_owned() + &image.url;
                    let thumb_path = img_dir.join(&new_thumb_url);
                    if let Ok(_) = thumb.save(&thumb_path)
                        && let Ok(orig_meta) = std::fs::metadata(&path)
                        && let Ok(thumb_meta) = std::fs::metadata(&thumb_path)
                        && (thumb_meta.len() as f32) / (orig_meta.len() as f32) < 0.75
                    {
                        thumb_url = new_thumb_url;
                        size.width = thumb.width() as usize;
                        size.height = thumb.height() as usize;
                    }
                }

                images.push(ImgInfo {
                    thumb_url,
                    url: image.url.clone(),
                    // title: image.title,
                    w: size.width,
                    h: size.height,
                });
                // println!("  GAL?: {} {:?} {:?}", image.url, image.title, imagesize::size(path));
            }

            node.replace(GalleryRow { images });
        });
    }
}

#[derive(Debug)]
struct ImgInfo {
    thumb_url: String,
    url: String,
    // title: Option<String>,
    w: usize,
    h: usize,
}

#[derive(Debug)]
pub struct GalleryRow {
    images: Vec<ImgInfo>,
}

impl NodeValue for GalleryRow {
    fn render(&self, _: &Node, fmt: &mut dyn Renderer) {
        fmt.open("div", &[("class", CLASS.to_owned())]);
        fmt.cr();
        for img in &self.images {
            fmt.open("a", &[
                ("href", img.url.clone()),
                ("style", format!("flex-grow:calc({}/{})", img.w, img.h)),
            ]);
            // FIXME: handle .title
            fmt.self_close("img", &[
                ("src", img.thumb_url.clone()),
                ("width", img.w.to_string()),
                ("height", img.h.to_string()),
            ]);
            fmt.close("a");
            fmt.cr();
        }
        fmt.close("div");
        fmt.cr();
    }
}
