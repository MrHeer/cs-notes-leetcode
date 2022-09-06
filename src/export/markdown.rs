use std::{fs::File, io::Write};

use super::Exporter;
use crate::api::{Category, Problem};

pub struct MarkdownExporter {
    file: File,
}

impl Exporter for MarkdownExporter {
    fn new(file: File) -> MarkdownExporter {
        MarkdownExporter { file }
    }

    fn export(&mut self, categories: &Vec<Category>) {
        let title = format_title();
        let categories = format_categories(categories);
        let document = [title, categories].join("\n");
        self.file.write(document.as_bytes()).unwrap();
    }
}

fn format_title() -> String {
    String::from("# CS-Notes Leetcode\n")
}

fn format_table_header() -> String {
    String::from("| Num | Problem | Difficulty |\n| --: | ------- | ---------- |")
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
    problems
        .iter()
        .map(format_problem)
        .collect::<Vec<String>>()
        .join("\n")
}

fn format_category(category: &Category) -> String {
    let Category { name, problems, .. } = category;

    let category_title = format_category_title(&name);
    let table_header = format_table_header();
    let problems = format_problems(&problems);

    [category_title, table_header, problems].join("\n")
}

fn format_categories(categories: &Vec<Category>) -> String {
    categories
        .iter()
        .map(format_category)
        .collect::<Vec<String>>()
        .join("\n\n")
}
