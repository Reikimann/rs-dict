use reqwest::{Client, Error, StatusCode};

pub async fn get_request(client: &Client, query: &str) -> Result<String, Error> {
    let url: String = "https://ordnet.dk/ddo/ordbog?query=".to_string() + &query;

    let response = client.get(url).send().await?;
    let status: StatusCode = response.status();

    if status.is_success() {
        return Ok(response.text().await?);
    }
    Err(response.error_for_status_ref().unwrap_err())
}
