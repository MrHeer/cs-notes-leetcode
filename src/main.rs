use api::fetch_categories;
use markdown::{print_categories, print_title};

mod api;
mod consts;
mod markdown;

fn main() {
    print_title();
    let categories = fetch_categories();
    print_categories(&categories);
}
