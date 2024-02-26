#[cfg(test)]
mod tests {
    use ordnet_dk::requests::get_request;

    #[tokio::test]
    async fn test_get_request_ok() {
        let client = reqwest::Client::new();
        let response = get_request(&client, "test").await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_get_request_err() {
        let client = reqwest::Client::new();
        let response = get_request(&client, "tasdfasdfasdgfasgadfgasdofest").await;

        assert!(response.is_err());
    }
}
