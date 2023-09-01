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
    // TODO: render list to _html/
    // TODO[LATER]: handle images
}
