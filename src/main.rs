#[macro_use]
extern crate lazy_static;

use datatypes::{mdn_data_types, DATA_TYPES_MAP};
use inquire::Text;

mod datatypes;
mod utils;

/**
 * Quick way to lookup on MDN
 * */
fn main() {
    println!("Enter a search query or filter and select an option\nTab to autocomplete");
    let query_result = Text::new("Query:")
        .with_validator(utils::validate_not_empty_text)
        .with_autocomplete(DATA_TYPES_MAP.clone())
        .prompt();

    let query = &match query_result {
        Ok(res) => res,
        Err(_) => {
            println!("Could not detect valid option selection. Please try again");
            return
        },
    };

    if let None = mdn_data_types(query) {
        mdn_search(query);
    }
}

const MDN_BASE_URL: &str = "https://developer.mozilla.org/en-US/search?q=";

fn mdn_search(search_query: &str) {
    let search_url = &format!(
        "{}{}",
        MDN_BASE_URL,
        search_query.replace("Search for ", "")
    )[..];

    utils::open_and_output_url(search_url)
}
