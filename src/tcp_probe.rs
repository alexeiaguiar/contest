use std::str::FromStr;
use crate::tcp_probe::TcpConnectionResult::{Connected, Refused, Timeout};
use tokio::net::TcpStream;
use tokio::time::{timeout};

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum TcpConnectionResult {
    Connected,
    Refused,
    Timeout
}

impl FromStr for TcpConnectionResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "connected" => Ok(Connected),
            "refused" => Ok(Refused),
            "timeout" => Ok(Timeout),
            _ => Err(()),
        }
    }
}

pub(crate) async fn tcp_probe(host: &str, port: u16) -> Result<TcpConnectionResult, Box<dyn std::error::Error>> {
    let addr = format!("{host}:{port}");
    let timeout_duration = std::time::Duration::from_secs(1);
    match timeout(timeout_duration, TcpStream::connect(addr)).await {
        Ok(Ok(_stream)) => Ok(Connected),
        Ok(Err(e)) if e.kind() == std::io::ErrorKind::ConnectionRefused => Ok(Refused),
        Ok(Err(e)) => Err(Box::new(e)),
        Err(_) => Ok(Timeout),
    }
}

#[cfg(test)]
mod tests {
    use crate::tcp_probe::TcpConnectionResult::Timeout;
    use super::*;

    #[tokio::test]
    async fn test_when_tcp_probe_then_connected() {
        assert_eq!(Connected, tcp_probe("google.ca", 80).await.unwrap());
    }

    #[tokio::test]
    async fn test_when_tcp_probe_then_timeout() {
        assert_eq!(Timeout, tcp_probe("google.ca", 81).await.unwrap());
    }

    #[tokio::test]
    async fn test_when_tcp_probe_then_refused() {
        assert_eq!(Refused, tcp_probe("localhost", 12345).await.unwrap());
    }

    #[test]
    fn test_when_parse_tcp_connection_result_then_connected() {
        assert_eq!(Connected, "connected".parse().unwrap());
    }

    #[test]
    fn test_when_parse_tcp_connection_result_then_refused() {
        assert_eq!(Refused, "refused".parse().unwrap());
    }

    #[test]
    fn test_when_parse_tcp_connection_result_then_timeout() {
        assert_eq!(Timeout, "timeout".parse().unwrap());
    }

    #[test]
    fn test_when_parse_tcp_connection_result_then_invalid() {
        assert!("invalid".parse::<TcpConnectionResult>().is_err());
    }
}