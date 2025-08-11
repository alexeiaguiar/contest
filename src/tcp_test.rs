use crate::config::Test;
use crate::tcp_test::TcpConnectionResult::{Connected, Refused, Timeout};
use serde::Deserialize;
use tokio::net::TcpStream;
use tokio::time::timeout;

#[derive(PartialEq, Eq, Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum TcpConnectionResult {
    Connected,
    Refused,
    Timeout,
}

#[derive(Debug, Deserialize)]
pub struct TcpTest {
    pub host: String,
    pub port: u16,
    pub expected: TcpConnectionResult,
    #[serde(skip, default)]
    pub actual: Option<TcpConnectionResult>,
}

impl Test for TcpTest {
    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", self.host, self.port);
        let timeout_duration = std::time::Duration::from_secs(1);
        match timeout(timeout_duration, TcpStream::connect(addr)).await {
            Ok(Ok(_stream)) => {
                self.actual = Some(Connected);
                Ok(())
            }
            Ok(Err(e)) if e.kind() == std::io::ErrorKind::ConnectionRefused => {
                self.actual = Some(Refused);
                Ok(())
            }
            Ok(Err(e)) => Err(Box::new(e)),
            Err(_) => {
                self.actual = Some(Timeout);
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
        format!(
            "{}  {} - Expected: {:?}, Actual: {:?}",
            emoji,
            test_name,
            self.expected,
            self.actual.as_ref().unwrap()
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tcp_test::TcpConnectionResult::Timeout;

    #[tokio::test]
    async fn test_when_tcp_probe_then_connected() {
        let mut test = TcpTest {
            host: "google.ca".to_string(),
            port: 80,
            expected: Connected,
            actual: None,
        };
        test.run().await.unwrap();
        assert_eq!(Connected, test.actual.unwrap());
    }

    #[tokio::test]
    async fn test_when_tcp_probe_then_timeout() {
        let mut test = TcpTest {
            host: "google.ca".to_string(),
            port: 81,
            expected: Timeout,
            actual: None,
        };
        test.run().await.unwrap();
        assert_eq!(Timeout, test.actual.unwrap());
    }

    #[tokio::test]
    async fn test_when_tcp_probe_then_refused() {
        let mut test = TcpTest {
            host: "localhost".to_string(),
            port: 12345,
            expected: Refused,
            actual: None,
        };
        test.run().await.unwrap();
        assert_eq!(Refused, test.actual.unwrap());
    }
}
