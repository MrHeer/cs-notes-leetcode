use std::{fs::File, io::Error};

use super::api::Category;

pub trait Exportor {
    fn new(file: File) -> Self;

    fn export(&mut self, categories: &Vec<Category>) -> Result<(), Error>;
}

pub mod markdown;
