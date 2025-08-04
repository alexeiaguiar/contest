use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TestConfig {
    pub tests: Vec<TestCase>,
}

#[derive(Debug, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub tcp: Option<TcpTest>,
    pub http: Option<HttpTest>,
}

#[derive(Debug, Deserialize)]
pub struct TcpTest {
    pub host: String,
    pub port: u16,
    pub expected: String,
}

#[derive(Debug, Deserialize)]
pub struct HttpTest {
    pub url: String,
    pub expected: u16,
}

pub(crate) fn read_config(file_path: &str) -> Result<TestConfig, Box<dyn Error>> {
    let yaml = std::fs::read_to_string(file_path)?;
    Ok(serde_yaml::from_str(&yaml).map_err(|e| Box::new(e))?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_parsing() {
        let config = read_config("test.yaml").unwrap();
        assert_eq!(config.tests.len(), 5);

        let test0= &config.tests[0];
        assert_eq!(test0.name, "TCP connected");
        let tcp0 = test0.tcp.as_ref().unwrap();
        assert_eq!(tcp0.host, "google.ca");
        assert_eq!(tcp0.port, 80);
        assert_eq!(tcp0.expected, "connected");

        let test1= &config.tests[1];
        assert_eq!(test1.name, "TCP timeout");
        let tcp1 = test1.tcp.as_ref().unwrap();
        assert_eq!(tcp1.host, "google.ca");
        assert_eq!(tcp1.port, 81);
        assert_eq!(tcp1.expected, "timeout");

        let test2= &config.tests[2];
        assert_eq!(test2.name, "HTTP 200");
        let http2 = test2.http.as_ref().unwrap();
        assert_eq!(http2.url, "https://httpbin.org/status/200");
        assert_eq!(http2.expected, 200);

        let test3= &config.tests[3];
        assert_eq!(test3.name, "HTTP 404");
        let http3 = test3.http.as_ref().unwrap();
        assert_eq!(http3.url, "https://httpbin.org/status/404");
        assert_eq!(http3.expected, 404);

        let test4= &config.tests[4];
        assert_eq!(test4.name, "HTTP 400");
        let http4 = test4.http.as_ref().unwrap();
        assert_eq!(http4.url, "https://httpbin.org/status/400");
        assert_eq!(http4.expected, 400);
    }
}