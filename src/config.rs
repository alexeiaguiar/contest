use crate::test_case::{TestCase, TestSummary};
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct TestConfig {
    pub parameters: Option<Parameters>,
    pub tests: Vec<TestCase>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Parameters {
    pub timeout_seconds: Option<u64>
}

pub trait Test {
    async fn run(&mut self, parameters: &Option<Parameters>) -> Result<(), Box<dyn Error>>;
    fn compare_results(&self, test_name: &str) -> TestSummary;
}

pub(crate) fn read_config(file_path: &str) -> Result<TestConfig, Box<dyn Error>> {
    let yaml = std::fs::read_to_string(file_path)?;
    Ok(serde_yaml::from_str(&yaml).map_err(Box::new)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tcp_test::TcpConnectionResult::{Connected, Refused, Timeout};

    #[test]
    fn test_complete_config_parsing() {
        let config = read_config("tests/complete-config.yaml").unwrap();

        assert_eq!(config.parameters, Some(Parameters { timeout_seconds: Some(5) }));

        assert_eq!(config.tests.len(), 9);

        let test_case = &config.tests[0];
        assert_eq!(test_case.name, "TCP connected");
        let tcp_test = test_case.tcp.as_ref().unwrap();
        assert_eq!(tcp_test.host, "google.ca");
        assert_eq!(tcp_test.port, 80);
        assert_eq!(tcp_test.expected, Connected);

        let test_case = &config.tests[1];
        assert_eq!(test_case.name, "TCP timeout");
        let tcp_test = test_case.tcp.as_ref().unwrap();
        assert_eq!(tcp_test.host, "google.ca");
        assert_eq!(tcp_test.port, 81);
        assert_eq!(tcp_test.expected, Timeout);

        let test_case = &config.tests[2];
        assert_eq!(test_case.name, "TCP refused");
        let tcp_test = test_case.tcp.as_ref().unwrap();
        assert_eq!(tcp_test.host, "localhost");
        assert_eq!(tcp_test.port, 12345);
        assert_eq!(tcp_test.expected, Refused);

        let test_case = &config.tests[3];
        assert_eq!(test_case.name, "HTTP 200");
        let http_test = test_case.http.as_ref().unwrap();
        assert_eq!(http_test.url, "https://postman-echo.com/status/200");
        assert_eq!(http_test.expected, Connected);
        assert_eq!(http_test.expected_status, Some(200));

        let test_case = &config.tests[4];
        assert_eq!(test_case.name, "HTTP 404");
        let http_test = test_case.http.as_ref().unwrap();
        assert_eq!(http_test.url, "https://postman-echo.com/status/404");
        assert_eq!(http_test.expected, Connected);
        assert_eq!(http_test.expected_status, Some(404));

        let test_case = &config.tests[5];
        assert_eq!(test_case.name, "HTTP timeout");
        let http_test = test_case.http.as_ref().unwrap();
        assert_eq!(http_test.url, "http://google.ca:81");
        assert_eq!(http_test.expected, Timeout);

        let test_case = &config.tests[6];
        assert_eq!(test_case.name, "HTTP refused");
        let http_test = test_case.http.as_ref().unwrap();
        assert_eq!(http_test.url, "http://localhost:12345");
        assert_eq!(http_test.expected, Refused);

        let test_case = &config.tests[7];
        assert_eq!(test_case.name, "Follow redirection");
        let http_test = test_case.http.as_ref().unwrap();
        assert_eq!(http_test.url, "https://sts.ca-central-1.amazonaws.com");
        assert_eq!(http_test.expected, Connected);
        assert_eq!(http_test.expected_status, Some(200));
        assert_eq!(http_test.redirect, None);

        let test_case = &config.tests[8];
        assert_eq!(test_case.name, "Do not follow redirection");
        let http_test = test_case.http.as_ref().unwrap();
        assert_eq!(http_test.url, "https://sts.ca-central-1.amazonaws.com");
        assert_eq!(http_test.expected, Connected);
        assert_eq!(http_test.expected_status, Some(302));
        assert_eq!(http_test.redirect, Some(false));
    }

    #[test]
    fn test_required_fields_config_parsing() {
        let config = read_config("tests/required-fields-config.yaml").unwrap();

        assert_eq!(config.parameters, None);

        assert_eq!(config.tests.len(), 1);

        let test_case = &config.tests[0];
        assert_eq!(test_case.name, "TCP connected");
        let tcp_test = test_case.tcp.as_ref().unwrap();
        assert_eq!(tcp_test.host, "google.ca");
        assert_eq!(tcp_test.port, 80);
        assert_eq!(tcp_test.expected, Connected);
    }

    #[test]
    fn test_empty_parameters_config_parsing() {
        let config = read_config("tests/empty-config.yaml").unwrap();

        assert_eq!(config.parameters, Some(Parameters { timeout_seconds: None }));
        assert_eq!(config.tests.len(), 0);
    }
}
