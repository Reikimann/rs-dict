// Reqwest and scraping
use reqwest::{Client, StatusCode};
use scraper::{Html, Selector};

// CLI and arg parsing
use std::{
    process,
    fmt::Write,
};
mod args;
use args::CliArgs;
use clap::Parser;
use colored::Colorize;

// Other
use ordnet_dk::requests::get_request;
mod utils;
use utils::{uppercase_first_letter, trim_whitespace};

lazy_static::lazy_static! {
    static ref HTTP_CLIENT: Client = Client::new();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();
    let query_word = args.query;

    let response: String = match get_request(&HTTP_CLIENT, &query_word).await {
        Ok(body) => body,
        Err(err) => {
            // TODO: Use human-panic.
            if err.status() == Some(StatusCode::NOT_FOUND) {
                eprintln!("Word doesn't exist.");
            }
            if err.is_connect() {
                eprintln!("Connection Error.");
            }
            process::exit(1)
        }
    };

    let document = Html::parse_document(&response);
    // Make these constants and make a scraping/selector module in ordnet_dk
    let selector_match = Selector::parse("div.definitionBoxTop > span.match").unwrap();
    let selector_glossing = Selector::parse("div.definitionBoxTop > span.tekstmedium.allow-glossing").unwrap();
    let selector_inflection = Selector::parse("div#id-boj > span.tekstmedium.allow-glossing").unwrap();
    let selector_pronounciation = Selector::parse("div#id-udt > span.tekstmedium.allow-glossing > .lydskrift").unwrap();
    let selector_etymologi = Selector::parse("div#id-ety > span.tekstmedium.allow-glossing").unwrap();

    let mut output = String::new();

    // Remember to check if the inner_html is empty and what text to display instead.
    let result_match: String = document
        .select(&selector_match)
        .map(|el| el.inner_html())
        .collect::<Vec<_>>()
        .join("/");

    write!(output, "{}\n", uppercase_first_letter(&result_match).blue().bold())?;


    // Ordklasse og køn
    for element in document.select(&selector_glossing) {
        let inner_html = element.inner_html();
        write!(output, "{}\n", inner_html)?;
    }

    // Bøjninger
    for element in document.select(&selector_inflection) {
        let text = element.text().collect::<Vec<_>>().join("");
        let header = "Bøjning:".blue().bold();
        write!(output, "\n{}\n{}\n", header, text)?;
    }

    // Udtale
    for element in document.select(&selector_pronounciation) {
        let text = element.text().collect::<Vec<_>>().join("");
        let header = "Udtale:".blue().bold();
        write!(output, "\n{}\n{}\n", header, text)?;
    }

    // Etymologi
    for element in document.select(&selector_etymologi) {
        let text = element.text().collect::<Vec<_>>().join("");
        let trimmed_text = trim_whitespace(&text);
        let header = "Udtale:".blue().bold();
        write!(output, "\n{}\n{}.\n", header, uppercase_first_letter(&trimmed_text))?;
    }


    println!("{}", output);

    Ok(())
}
