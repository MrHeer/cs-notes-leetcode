use std::fs::File;

use api::fetch_categories;
use export::{markdown::MarkdownExportor, Exportor};

mod api;
mod consts;
mod export;

fn main() {
    println!("Fetching data...");
    let categories = fetch_categories();

    let file_name = "README.md";
    println!("Writing data to {}...", file_name);
    let file = File::create(file_name).unwrap();
    let mut exportor = MarkdownExportor::new(file);
    exportor.export(&categories);
    println!("Done!");
}
