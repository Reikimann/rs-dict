// Reqwest and scraping
use reqwest::{Client, StatusCode};
use scraper::{
    Html,
    Selector,
    Element,
    selector::CssLocalName,
    CaseSensitivity,
};

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
    let selector_pronounciation = Selector::parse("div#id-udt > span.tekstmedium.allow-glossing").unwrap();
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
    let result_glossing: String = document
        .select(&selector_glossing)
        .map(|el| el.inner_html())
        .collect::<Vec<_>>()
        .join("");

    write!(output, "{}\n", uppercase_first_letter(&result_glossing))?;

    // Bøjninger
    let result_inflection: String = document
        .select(&selector_inflection)
        .map(|el| {
            el.text().collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("");

    let header = "Bøjninger:".blue().bold();
    write!(output, "\n{}\n{}\n", header, &result_inflection)?;

    // Udtale
    // let result_pronounciation = document
    //     .select(&selector_pronounciation)
    //     .map(|el| {
    //         // el.text()
    //         let text = el.text().collect::<String>();
    //         let class = CssLocalName("dividerDouble".into());
    //         if el.has_class(&class, CaseSensitivity::CaseSensitive) {
    //             format!(" eller {}", text)
    //         } else {
    //             format!(" {}", text)
    //         }
    //     })
    //     .collect::<Vec<_>>()
    //     .join("");

    let divider_class = CssLocalName("dividerDouble".into());
    let lydskrift_class = CssLocalName("lydskrift".into());
    let selector_span = Selector::parse("span").unwrap();

    let result_pronounciation = document
        .select(&selector_pronounciation)
        .map(|el| el.inner_html())
        .map(|el| {
            let fragment = Html::parse_fragment(&el);

            let inner_texts: Vec<_> = fragment
                .select(&selector_span)
                .map(|el| {
                    let mut text = el.text().collect::<String>()
                        .replace("[", "").to_string()
                        .replace("]", "").to_string();

                    if !text.is_empty() {
                        text = text.replace("\u{a0}", "").to_string();
                        if el.has_class(&divider_class, CaseSensitivity::CaseSensitive) {
                            format!(" eller {}", text)
                        } else if el.has_class(&lydskrift_class, CaseSensitivity::CaseSensitive) {
                            format!(" [{}]", text)
                        } else {
                            format!(" {}", text)
                        }
                    } else {
                        String::new()
                    }
                })
                .collect();

            inner_texts.join("")
        })
        .collect::<Vec<_>>()
        .join("");
    // for element in document.select(&selector_pronounciation) {
    //     let text = element.text().collect::<Vec<_>>().join("");
    //     let header = "Udtale:".blue().bold();
    //     write!(output, "\n{}\n{}\n", header, text)?;
    // }
    let header = "Udtale:".blue().bold();
    write!(output, "\n{}\n{}\n", header, trim_whitespace(&result_pronounciation))?;

    // Etymologi
    for element in document.select(&selector_etymologi) {
        let text = element.text().collect::<Vec<_>>().join("");
        let trimmed_text = trim_whitespace(&text);
        let header = "Oprindelse:".blue().bold();
        write!(output, "\n{}\n{}.\n", header, uppercase_first_letter(&trimmed_text))?;
    }


    println!("{}", output);

    Ok(())
}
