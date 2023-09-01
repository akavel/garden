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

const BASE_SOURCES: &str = "*.md";
const DRAFT_SOURCES: &str = "_drafts/*.md";
const OUT_DIR: &str = "_html";

fn main() {
    logging::init_info();
    info!("👋😃");

    // TODO: load sources: *.md & _drafts/*.md
    info!("Scanning {BASE_SOURCES} & {DRAFT_SOURCES}");
    let paths: Vec<_> = [BASE_SOURCES, DRAFT_SOURCES]
        .iter()
        .flat_map(|s| glob(s).unwrap())
        .flat_map(|maybe_path| {
            // TODO: use inspect_err when stabilized
            maybe_path.map_err(|e| {
                warn!("{e}");
            })
        })
        .collect();
    println!("{paths:?}");
    // TODO: parse filename (slug, tags, date, file extension)
    // -< PEG or parser-combinator, ideally with DSL/macro/annotation
    // TODO: render articles to _html/
    // TODO: render list to _html/
    // TODO[LATER]: handle images
}


