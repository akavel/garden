use log::info;

mod logging;

fn main() {
    logging::init_info();
    info!("👋😃");

    // TODO: load sources: *.md & _drafts/*.md
    // TODO: parse filename (slug, tags, date, file extension)
    // -< PEG or parser-combinator, ideally with DSL/macro/annotation
    // TODO: render articles
    // TODO: render list
}
