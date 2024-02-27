// Reqwest and scraping
use reqwest::{Client, StatusCode};
use scraper::Html;

// CLI and arg parsing
use std::{
    process,
    fmt::Write,
};
mod args;
use args::CliArgs;
use clap::Parser;

// Other
use ordnet_dk::requests::get_request;
use ordnet_dk::parser;
mod utils;
use utils::{uppercase_first_letter, format_header};

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
    let mut output = String::new();
    let mut header: String;

    // Match
    let result_match: String = parser::get_match(&document);
    write!(output, "{}\n", format_header(&uppercase_first_letter(&result_match)))?;

    // Ordklasse og køn
    let result_glossing: String = parser::get_glossing(&document);
    write!(output, "{}\n", uppercase_first_letter(&result_glossing))?;

    // Bøjninger
    let result_inflection: String = parser::get_inflection(&document);
    header = format_header("Bøjninger:");
    write!(output, "\n{}\n{}\n", header, &result_inflection)?;

    // Udtale
    let result_pronounciation: String = parser::get_pronounciation(&document);
    header = format_header("Udtale:");
    write!(output, "\n{}\n{}\n", header, &result_pronounciation)?;

    // Etymologi
    let result_etymology: String = parser::get_etymology(&document);
    header = format_header("Etymologi:");
    write!(output, "\n{}\n{}.", header, uppercase_first_letter(&result_etymology))?;

    println!("{}", output);

    Ok(())
}
