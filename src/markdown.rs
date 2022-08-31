use crate::api::{Category, Problem};

pub fn print_title() {
    println!("# CS-Notes Leetcode");
    println!();
}

fn print_tabel_header() {
    println!("| Num | Problem | Difficulty |");
    println!("| --: | ------- | ---------- |");
}

fn print_category_title(category_name: &str) {
    println!("## {}", category_name);
    println!();
}

fn print_problem(problem: &Problem) {
    let Problem {
        number,
        name,
        url,
        diff,
    } = problem;
    println!("|{}|[{}]({})|`{}`|", number, name, url, diff);
}

fn print_problems(problems: &Vec<Problem>) {
    problems.iter().for_each(print_problem);
    println!();
}

fn print_category(category: &Category) {
    print_category_title(&category.name);
    print_tabel_header();
    print_problems(&category.problems);
}

pub fn print_categories(categories: &Vec<Category>) {
    categories.iter().for_each(print_category);
}
