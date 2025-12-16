use crate::config::{Parameters, Test};
use crate::tcp_test::TcpConnectionResult;
use crate::tcp_test::TcpConnectionResult::{Connected, Refused, Timeout};
use reqwest::StatusCode;
use serde::Deserialize;
use std::time::Duration;
use reqwest::redirect::Policy;
use crate::test_case::TestSummary;
use crate::test_case::TestResult::{Fail, Pass};

#[derive(Debug, Deserialize)]
pub struct HttpTest {
    pub url: String,
    pub expected: TcpConnectionResult,
    pub expected_status: Option<u16>,
    pub redirect: Option<bool>,
    #[serde(skip, default)]
    pub actual: Option<TcpConnectionResult>,
    #[serde(skip, default)]
    pub actual_status: Option<StatusCode>,
}

impl Test for HttpTest {
    async fn run(&mut self, parameters: &Option<Parameters>) -> Result<(), Box<dyn std::error::Error>> {
        let redirect_policy = if self.redirect.unwrap_or(true) {
            Policy::default()
        } else {
            Policy::none()
        };

        let client_builder = reqwest::Client::builder()
            .redirect(redirect_policy);

        let client = match parameters.as_ref().and_then(|p| p.timeout_seconds) {
            None => client_builder,
            Some(timeout_seconds) => client_builder.timeout(Duration::from_secs(timeout_seconds))
        }.build()?;

        match client.get(&self.url).send().await {
            Ok(response) => {
                self.actual = Some(Connected);
                self.actual_status = Some(response.status());
                Ok(())
            }
            Err(e) if e.is_timeout() => {
                self.actual = Some(Timeout);
                self.actual_status = None;
                Ok(())
            }
            Err(_) => {
                self.actual = Some(Refused);
                self.actual_status = None;
                Ok(())
            }
        }
    }
    fn compare_results(&self, test_name: &str) -> TestSummary {
        let actual = self.actual.as_ref().expect("Test has not been run yet");

        let pass = self.expected == *actual && self.expected_status == self.actual_status.map(|s| s.as_u16());

        let emoji = if pass {
            "✅  Pass"
        } else {
            "❌  Fail"
        };

        let details = match &self.expected {
            Connected => match self.actual.as_ref().unwrap() {
                Connected => format!(
                    "{} - {} Expected: Connected with status {}, Actual: Connected with status {:?}",
                    emoji,
                    test_name,
                    self.expected_status.unwrap(),
                    self.actual_status.unwrap()
                ),
                _ => format!(
                    "{} - {} Expected: Connected, Actual: {:?}",
                    emoji,
                    test_name,
                    self.actual.as_ref().unwrap()
                ),
            },
            Refused | Timeout => format!(
                "{} - {} Expected: {:?}, Actual: {:?}",
                emoji, test_name, self.expected, self.actual.as_ref().unwrap()
            ),
        };

        TestSummary {
            result: if pass {
                Pass
            } else {
                Fail
            },
            details
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{Test};
    use crate::http_test::HttpTest;
    use crate::tcp_test::TcpConnectionResult::{Connected, Refused, Timeout};
    use reqwest::StatusCode;

    #[tokio::test]
    async fn test_when_http_probe_then_success() {
        let mut test = HttpTest {
            url: "https://postman-echo.com/status/200".to_string(),
            expected: Connected,
            expected_status: Some(200),
            redirect: None,
            actual: None,
            actual_status: None,
        };

        test.run(&None).await.unwrap();
        assert_eq!(Connected, test.actual.unwrap());
        assert_eq!(Some(StatusCode::OK), test.actual_status);
    }

    #[tokio::test]
    async fn test_when_http_probe_then_not_found() {
        let mut test = HttpTest {
            url: "https://postman-echo.com/status/404".to_string(),
            expected: Connected,
            expected_status: Some(404),
            redirect: None,
            actual: None,
            actual_status: None,
        };
        test.run(&None).await.unwrap();
        assert_eq!(Connected, test.actual.unwrap());
        assert_eq!(Some(StatusCode::NOT_FOUND), test.actual_status);
    }

    #[tokio::test]
    async fn test_when_http_probe_then_timeout() {
        let mut test = HttpTest {
            url: "http://google.ca:81".to_string(),
            expected: Timeout,
            expected_status: None,
            redirect: None,
            actual: None,
            actual_status: None,
        };
        let parameters = Some(crate::config::Parameters {
            timeout_seconds: Some(1),
        });
        test.run(&parameters).await.unwrap();
        assert_eq!(Timeout, test.actual.unwrap());
        assert_eq!(None, test.actual_status);
    }

    #[tokio::test]
    async fn test_when_http_probe_then_refused() {
        let mut test = HttpTest {
            url: "http://localhost:12345".to_string(),
            expected: Refused,
            expected_status: None,
            redirect: None,
            actual: None,
            actual_status: None,
        };
        test.run(&None).await.unwrap();
        assert_eq!(Refused, test.actual.unwrap());
        assert_eq!(None, test.actual_status);
    }
}
