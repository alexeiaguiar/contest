use std::fmt::Debug;

mod tcp_probe;
mod http_probe;

fn compare_results<T: PartialEq + Debug>(name: &str, expected: T, actual: T) {
    if expected == actual {
        print!("✅");
    } else {
        print!("❌");
    }
    println!("  {name} Expected: {expected:?}, Actual: {actual:?}");
}

mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = std::env::args().nth(1).expect("Please provide a config file path as the first argument");

    println!("Running tests from config file: {file_path}");

    let config = crate::config::read_config(&file_path)?;
    for test in config.tests {
        if let Some(tcp) = test.tcp {
            let result = tcp_probe::tcp_probe(&tcp.host, tcp.port).await?;
            // FIXME: Remove this unwrap once the TcpConnectionResult is implemented
            let expected = tcp.expected.parse().unwrap();
            compare_results(&test.name, expected, result);
        }
        else if let Some(http) = test.http {
            let status = http_probe::http_probe(&http.url).await?;
            compare_results(&test.name, http.expected, status.as_u16());
        }
    }

    Ok(())
}
