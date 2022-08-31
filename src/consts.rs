use lazy_static::lazy_static;
use regex::Regex;

pub const BASE_URL: &str = "http://www.cyc2018.xyz";

pub const LEETCODE_URL: &str =
    const_str::concat!(BASE_URL, "/算法/Leetcode 题解/Leetcode 题解 - 目录.html");

lazy_static! {
    pub static ref PROBLEM_RE: Regex =
        Regex::new(r"^(?P<number>\d+?)\. (?P<name>.+?) \((?P<diff>\w+)\)$").unwrap();
}

lazy_static! {
    pub static ref CATEGORY_NAME_RE: Regex = Regex::new(r"^.+ - (?P<category>.+)\.html$").unwrap();
}
