use std::{error, fmt::Display};

use const_str::parse;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::blocking;
use scraper::{Html, Selector};

mod consts;

#[derive(Debug, PartialEq, Eq)]
enum Diff {
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

#[derive(Debug, PartialEq, Eq)]
struct Problem {
    number: usize,
    name: String,
    diff: Diff,
    url: String,
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "|{}|[{}]({})|`{}`|",
            self.number, self.name, self.url, self.diff
        )
    }
}

fn fetch(url: &str) -> Result<Html, Box<dyn error::Error>> {
    let resp = blocking::get(url)?.text().unwrap();
    let html = Html::parse_document(&resp);
    Ok(html)
}

fn fetch_problems(url: &str) -> Vec<Problem> {
    let html = fetch(url).unwrap();
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(?P<number>\d+?)\. (?P<name>.+?) \((?P<diff>\w+)\)$").unwrap();
    }
    let leetcode_title_selector = Selector::parse("main h2 + p, main h3 + p").unwrap();
    let title_elements = html
        .select(&leetcode_title_selector)
        .filter(|title_element| RE.is_match(&title_element.inner_html()));
    let mut problems: Vec<Problem> = Vec::new();
    title_elements.for_each(|title_element| {
        let url_element = title_element
            .next_siblings()
            .nth(1)
            .unwrap()
            .first_child()
            .unwrap()
            .value()
            .as_element()
            .unwrap();
        let url = url_element.attr("href").unwrap();

        let title = title_element.inner_html();
        let caps = RE.captures(&title).unwrap();
        let problem = Problem {
            number: parse!(caps.name("number").unwrap().as_str(), usize),
            name: caps.name("name").unwrap().as_str().to_string(),
            diff: match caps.name("diff").unwrap().as_str() {
                "Medium" => Diff::Medium,
                "Hard" => Diff::Hard,
                _ => Diff::Easy,
            },
            url: url.to_string(),
        };
        problems.push(problem);
    });
    problems
}

fn print_title() {
    println!("# CS-Notes Leetcode");
    println!();
}

fn print_chapter_header(chapter: &str) {
    println!("## {}", chapter);
    println!();
    println!("| Num | Problem | Difficulty |");
    println!("| --: | ------- | ---------- |");
}

fn print_problems(problems: &Vec<Problem>) {
    problems.iter().for_each(|problem| println!("{}", problem));
    println!();
}

fn main() {
    print_title();

    let chapter_link_selector = Selector::parse("main ul > li > a").unwrap();
    let html = fetch(consts::LEETCODE_URL).unwrap();
    let chapter_links = html.select(&chapter_link_selector);
    chapter_links.for_each(|chapter_element| {
        let chapter_url = chapter_element.value().attr("href").unwrap();
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^.+ - (?P<chapter>.+)\.html$").unwrap();
        }
        let chapter = RE
            .captures(chapter_url)
            .unwrap()
            .name("chapter")
            .unwrap()
            .as_str();

        print_chapter_header(chapter);

        let problems =
            fetch_problems((String::from(consts::BASE_URL) + &String::from(chapter_url)).as_str());
        print_problems(&problems);
    })
}
