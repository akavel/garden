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


