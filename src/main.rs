use std::fs::File;

use api::fetch_categories;
use export::{markdown::MarkdownExporter, Exporter};

mod api;
mod export;
mod statics;

fn main() {
    println!("Fetching data...");
    let categories = fetch_categories();

    let file_name = "README.md";
    println!("Writing data to {}...", file_name);
    let file = File::create(file_name).unwrap();
    let mut exporter = MarkdownExporter::new(file);
    exporter.export(&categories);
    println!("Done!");
}
