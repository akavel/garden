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
use log::{info, warn};
use std::path::Path;

mod logging;

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
        .map(|result| result.map(|p| PathInfo::parse(&p)))
        .flat_map(|maybe_path| {
            // TODO: use inspect_err when stabilized
            maybe_path.map_err(|e| {
                warn!("{e}");
            })
        })
        .collect();
    println!("{paths:?}");

    // TODO: parse filename (slug, tags, date, file extension)
    // "_drafts/NNNN-NN-NN-foo-bar.md"
    // "NNNNNNNNNN.@foo-bar.@baz.md"
    // -< PEG or parser-combinator, ideally with DSL/macro/annotation
    // TODO: render articles to _html/
    // TODO: render list to _html/
    // TODO[LATER]: handle images
}

/// Date and time digits, presumed in big-endian order.
/// Precision unspecified (may be YYYYMMDD, or just YYYYMM, or YYYYMMDDHHMM, etc.).
#[derive(Debug)]
struct DateTime(String);

#[derive(Debug)]
struct PathInfo {
    slug: String,
    datetime: DateTime,
    tags: Vec<String>,
    extension: String,
}

impl PathInfo {
    fn parse(path: &Path) -> Result<PathInfo, PathInfoError> {
        if path.file_name().is_none() {
            return Err(PathInfoError::MissingFileName);
        }
        use std::path::Component;
        let utf8_dir_tags: Option<Vec<String>> = path
            .components()
            .rev()
            .skip(1)
            .filter_map(|c| match c {
                Component::Normal(s) => Some(s),
                _ => None,
            })
            .map(|s| s.to_str().map(String::from))
            .collect();
        let Some(dir_tags) = utf8_dir_tags else {
            return Err(PathInfoError::NotUTF8);
        };
        let Some(stem) = path.file_stem().unwrap_or_default().to_str() else {
            return Err(PathInfoError::NotUTF8);
        };
        let slug = stem.to_string(); // FIXME
        let datetime = DateTime(String::default()); // FIXME
        let tags = dir_tags; // FIXME
        let Some(extension) = path.extension().unwrap_or_default().to_str() else {
            return Err(PathInfoError::NotUTF8);
        };
        let extension = extension.to_string();
        Ok(PathInfo {
            slug,
            datetime,
            tags,
            extension,
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PathInfoError {
    #[error("file name is missing")]
    MissingFileName,
    #[error("path does not conform to UTF-8")]
    NotUTF8,
}
