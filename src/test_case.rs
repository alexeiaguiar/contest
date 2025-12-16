use crate::config::{Parameters, Test};
use crate::http_test::HttpTest;
use crate::tcp_test::TcpTest;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub tcp: Option<TcpTest>,
    pub http: Option<HttpTest>,
}

#[derive(PartialEq, Eq)]
pub enum TestResult {
    Pass,
    Fail,
}

pub struct TestSummary {
    pub result: TestResult,
    pub details: String,
}

impl TestCase {
    pub async fn run(&mut self, parameters: &Option<Parameters>) -> Result<(), Box<dyn Error>> {
        let none_type = self.tcp.is_none() && self.http.is_none();
        let both_types = self.tcp.is_some() && self.http.is_some();
        if none_type || both_types {
            return Err(Box::from(
                "Test case must have either a TCP or HTTP test defined",
            ));
        }

        if let Some(tcp_test) = &mut self.tcp {
            tcp_test.run(parameters).await?;
        }
        if let Some(http_test) = &mut self.http {
            http_test.run(parameters).await?;
        }
        Ok(())
    }

    pub fn compare_results(&self) -> TestSummary {
        if let Some(tcp_test) = &self.tcp {
            tcp_test.compare_results(&self.name)
        } else if let Some(http_test) = &self.http {
            http_test.compare_results(&self.name)
        } else {
            // This case should never happen due to the checks in run()
            panic!("Test case must have either a TCP or HTTP test defined")
        }
    }
}
