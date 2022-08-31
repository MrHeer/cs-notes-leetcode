use std::{fs::File, io::Write};

use super::Exportor;
use crate::api::{Category, Problem};

pub struct MarkdownExportor {
    file: File,
}

impl Exportor for MarkdownExportor {
    fn new(file: File) -> MarkdownExportor {
        MarkdownExportor { file }
    }

    fn export(&mut self, categories: &Vec<Category>) -> Result<(), std::io::Error> {
        let title = format_title();
        let categories = format_categories(categories);
        let document = [title, categories].join("\n");
        self.file.write(document.as_bytes())?;
        Ok(())
    }
}

fn format_title() -> String {
    String::from("# CS-Notes Leetcode\n")
}

fn format_table_header() -> String {
    format!("| Num | Problem | Difficulty |\n| --: | ------- | ---------- |",)
}

fn format_category_title(category_name: &str) -> String {
    format!("## {}\n", category_name)
}

fn format_problem(problem: &Problem) -> String {
    let Problem {
        number,
        name,
        url,
        diff,
    } = problem;
    format!("|{}|[{}]({})|`{}`|", number, name, url, diff)
}

fn format_problems(problems: &Vec<Problem>) -> String {
    let mut buf = Vec::new();
    problems.iter().for_each(|problem| {
        buf.push(format_problem(problem));
    });
    buf.join("\n")
}

fn format_category(category: &Category) -> String {
    let Category { name, problems, .. } = category;

    let category_title = format_category_title(&name);
    let table_header = format_table_header();
    let problems = format_problems(&problems);

    [category_title, table_header, problems].join("\n")
}

fn format_categories(categories: &Vec<Category>) -> String {
    let mut buf = Vec::new();
    categories.iter().for_each(|category| {
        buf.push(format_category(category));
    });
    buf.join("\n\n")
}
