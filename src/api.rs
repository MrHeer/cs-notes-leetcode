use std::fmt::Display;

use super::statics::{BASE_URL, CATEGORY_NAME_RE, LEETCODE_URL, PROBLEM_RE};
use const_str::parse;
use reqwest::blocking;
use scraper::{node::Element, ElementRef, Html, Selector};

#[derive(Debug, Clone)]
pub enum Diff {
    Easy,
    Medium,
    Hard,
}

impl Display for Diff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Easy => "Easy",
            Self::Medium => "Medium",
            Self::Hard => "Hard",
        };
        write!(f, "{}", text)
    }
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub number: usize,
    pub name: String,
    pub diff: Diff,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct Category {
    pub name: String,
    #[allow(unused)]
    pub url: String,
    pub problems: Vec<Problem>,
}

fn fetch(url: &str) -> Html {
    let resp = blocking::get(url).unwrap().text().unwrap();
    Html::parse_document(&resp)
}

fn find_url_element(problem_element: &ElementRef) -> Element {
    problem_element
        .next_siblings()
        .nth(1)
        .unwrap()
        .first_child()
        .unwrap()
        .value()
        .as_element()
        .unwrap()
        .to_owned()
}

fn make_problem(problem_element: &ElementRef) -> Problem {
    let url_element = find_url_element(&problem_element);

    let title = problem_element.inner_html();
    let caps = PROBLEM_RE.captures(&title).unwrap();

    let number = parse!(caps.name("number").unwrap().as_str(), usize);
    let name = caps.name("name").unwrap().as_str().to_string();
    let url = url_element.attr("href").unwrap().to_string();
    let diff = match caps.name("diff").unwrap().as_str() {
        "Medium" => Diff::Medium,
        "Hard" => Diff::Hard,
        _ => Diff::Easy,
    };

    Problem {
        number,
        name,
        diff,
        url,
    }
}

fn fetch_problems(url: &str) -> Vec<Problem> {
    let html = fetch(url);
    let problem_selector = Selector::parse("main h2 + p, main h3 + p").unwrap();
    let problem_elements = html
        .select(&problem_selector)
        .filter(|problem_element| PROBLEM_RE.is_match(&problem_element.inner_html()));
    let mut problems: Vec<Problem> = Vec::new();
    problem_elements.for_each(|problem_element| {
        problems.push(make_problem(&problem_element));
    });
    problems
}

fn make_category(category_element: &ElementRef) -> Category {
    let url = category_element.value().attr("href").unwrap().to_string();
    let name = CATEGORY_NAME_RE
        .captures(&url)
        .unwrap()
        .name("category")
        .unwrap()
        .as_str()
        .to_string();

    let problems = fetch_problems(&(BASE_URL.to_owned() + &url));

    Category {
        name: name.to_string(),
        url: url.to_string(),
        problems,
    }
}

pub fn fetch_categories() -> Vec<Category> {
    let mut categories = Vec::new();

    let categories_link_selector = Selector::parse("main ul > li > a").unwrap();
    let html = fetch(LEETCODE_URL);
    let categories_links = html.select(&categories_link_selector);
    categories_links.for_each(|category_element| {
        categories.push(make_category(&category_element));
    });
    categories
}
