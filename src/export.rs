use std::fs::File;

use super::api::Category;

pub trait Exportor {
    fn new(file: File) -> Self;

    fn export(&mut self, categories: &Vec<Category>);
}

pub mod markdown;
