use std::fs::File;

use api::fetch_categories;
use export::{markdown::MarkdownExportor, Exportor};

mod api;
mod consts;
mod export;

fn main() {
    let categories = fetch_categories();
    let file = File::create("README.md").unwrap();
    let mut export = MarkdownExportor::new(file);
    export.export(&categories);
}
