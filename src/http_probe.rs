use std::time::Duration;
use reqwest::StatusCode;

pub(crate) async fn http_probe(url: &str) -> Result<StatusCode, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()?;
    let response = client.get(url).send().await?;
    Ok(response.status())
}

#[cfg(test)]
mod tests {
    use reqwest::StatusCode;
    use crate::http_probe::http_probe;

    #[tokio::test]
    async fn test_when_http_probe_then_success() {
        assert_eq!(StatusCode::OK, http_probe("https://httpbin.org/status/200").await.unwrap());
    }

    #[tokio::test]
    async fn test_when_http_probe_then_not_found() {
        assert_eq!(StatusCode::NOT_FOUND, http_probe("https://httpbin.org/status/404").await.unwrap());
    }

    #[tokio::test]
    async fn test_when_http_probe_then_timeout() {
        assert_eq!(StatusCode::BAD_REQUEST, http_probe("https://httpbin.org/status/400").await.unwrap());
    }
}