use anyhow::Result;
use colored::*;
use deployer::Deployer;

fn main() -> Result<()> {
    let config = match std::env::args().skip(1).next() {
        Some(path) => path,
        None => "deployer.yml".to_string(),
    };

    let banner = r##"

█▀▄ █▀▀ █▀█ █░░ █▀█ █▄█ █▀▀ █▀█
█▄▀ ██▄ █▀▀ █▄▄ █▄█ ░█░ ██▄ █▀▄
           "##;
    println!("{}", banner.red().bold());

    let mut deployer = Deployer::new();
    deployer.configure(config.as_str());
    deployer.deploy();
    Ok(())
}
