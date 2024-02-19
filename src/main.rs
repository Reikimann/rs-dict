// use scraper::{Html, Selector};
use reqwest::{Client, Error, StatusCode};

mod args;
use args::CliArgs;
use clap::Parser;

lazy_static::lazy_static! {
    static ref HTTP_CLIENT: Client = Client::new();
}

async fn get_request(url: &str) -> Result<String, Error> {
    let response = HTTP_CLIENT.get(url).send().await?;
    let status: StatusCode = response.status();

    if status.is_success() {
        return Ok(response.text().await?);
    }
    Err(response.error_for_status_ref().unwrap_err())
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let query_word = args.query.unwrap_or_default();
    let query_url: String = "https://ordnet.dk/ddo/ordbog?query=".to_string() + &query_word;
    println!("{}\n", &query_url);

    match get_request(&query_url).await {
        Ok(body) => println!("{:#?}", body),
        Err(err) => eprintln!("Error: {}", err),
    }
}


// Synchronous
// use scraper::{Html, Selector};
// use reqwest::Error;
// use reqwest::blocking::{Client, Response};

// lazy_static::lazy_static! {
//     static ref HTTP_CLIENT: Client = Client::new();
// }

// // May return a string
// fn get_request(url: &str) -> Result<String, Error> {
//     let http_result: Result<Response, Error> = HTTP_CLIENT.get(url).send();

//     if http_result.is_ok() {
//         let body = http_result?.text()?;
//         println!("Success! Body:\n{}", body);
//         Ok(body)
//     } else {
//         Err(http_result?.error_for_status_ref().unwrap_err())
//     }
// }

// fn main() {
//     let res = get_request("https://ordnet.dk/ddo/ordbog?query=test");
//     println!("Body: {:#?}", res);
// }

