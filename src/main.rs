use std::{fs::File, io::Error};

use api::fetch_categories;
use export::{markdown::MarkdownExportor, Exportor};

mod api;
mod consts;
mod export;

fn main() -> Result<(), Error> {
    let categories = fetch_categories();
    let file = File::create("README_TEST.md")?;
    let mut export = MarkdownExportor::new(file);
    export.export(&categories)?;
    Ok(())
}
