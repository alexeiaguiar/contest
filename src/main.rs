mod config;
mod http_test;
mod tcp_test;
mod test_case;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Path to the config file
    #[arg(value_name = "CONFIG_FILE")]
    config_file: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let cli = Cli::parse();
    println!("Running tests from config file: {}", cli.config_file);

    let config = config::read_config(&cli.config_file)?;
    for mut test in config.tests {
        test.run().await?;
        println!("{}", test.compare_results());
    }

    Ok(())
}
