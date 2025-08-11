use crate::config::Test;
use crate::tcp_test::TcpConnectionResult;
use crate::tcp_test::TcpConnectionResult::{Connected, Refused, Timeout};
use reqwest::StatusCode;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct HttpTest {
    pub url: String,
    pub expected: TcpConnectionResult,
    pub expected_status: Option<u16>,
    #[serde(skip, default)]
    pub actual: Option<TcpConnectionResult>,
    #[serde(skip, default)]
    pub actual_status: Option<StatusCode>,
}

impl Test for HttpTest {
    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(2))
            .build()?;
        match client.get(&self.url).send().await {
            Ok(response) => {
                self.actual = Some(Connected);
                self.actual_status = Some(response.status());
                Ok(())
            }
            Err(e) if e.is_timeout() => {
                self.actual = Some(TcpConnectionResult::Timeout);
                self.actual_status = None;
                Ok(())
            }
            Err(_) => {
                self.actual = Some(TcpConnectionResult::Refused);
                self.actual_status = None;
                Ok(())
            }
        }
    }
    fn compare_results(&self, test_name: &str) -> String {
        let actual = self.actual.as_ref().expect("Test has not been run yet");
        let emoji = if self.expected == *actual {
            "✅"
        } else {
            "❌"
        };

        match &self.expected {
            Connected => match self.actual.as_ref().unwrap() {
                Connected => format!(
                    "{}  {} Expected: Connected with status {}, Actual: Connected with status {:?}",
                    emoji,
                    test_name,
                    self.expected_status.unwrap(),
                    self.actual_status.unwrap()
                ),
                _ => format!(
                    "{}  {} Expected: Connected, Actual: {:?}",
                    emoji,
                    test_name,
                    self.actual.as_ref().unwrap()
                ),
            },
            Refused | Timeout => format!(
                "{}  {} Expected: {:?}, Actual: {:?}",
                emoji, test_name, self.expected, self.actual.as_ref().unwrap()
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Test;
    use crate::http_test::HttpTest;
    use crate::tcp_test::TcpConnectionResult::{Connected, Refused, Timeout};
    use reqwest::StatusCode;

    #[tokio::test]
    async fn test_when_http_probe_then_success() {
        let mut test = HttpTest {
            url: "https://httpbin.org/status/200".to_string(),
            expected: Connected,
            expected_status: Some(200),
            actual: None,
            actual_status: None,
        };
        test.run().await.unwrap();
        assert_eq!(Connected, test.actual.unwrap());
        assert_eq!(Some(StatusCode::OK), test.actual_status);
    }

    #[tokio::test]
    async fn test_when_http_probe_then_not_found() {
        let mut test = HttpTest {
            url: "https://httpbin.org/status/404".to_string(),
            expected: Connected,
            expected_status: Some(404),
            actual: None,
            actual_status: None,
        };
        test.run().await.unwrap();
        assert_eq!(Connected, test.actual.unwrap());
        assert_eq!(Some(StatusCode::NOT_FOUND), test.actual_status);
    }

    #[tokio::test]
    async fn test_when_http_probe_then_timeout() {
        let mut test = HttpTest {
            url: "http://google.ca:81".to_string(),
            expected: Timeout,
            expected_status: None,
            actual: None,
            actual_status: None,
        };
        test.run().await.unwrap();
        assert_eq!(Timeout, test.actual.unwrap());
        assert_eq!(None, test.actual_status);
    }

    #[tokio::test]
    async fn test_when_http_probe_then_refused() {
        let mut test = HttpTest {
            url: "http://localhost:12345".to_string(),
            expected: Refused,
            expected_status: None,
            actual: None,
            actual_status: None,
        };
        test.run().await.unwrap();
        assert_eq!(Refused, test.actual.unwrap());
        assert_eq!(None, test.actual_status);
    }
}
