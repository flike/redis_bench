mod bench;

use bench::client;
use bench::config::BenchConfig;
use clap::Parser;
use config::{Config as AppConfig, File as AppFile};
use log::*;
use simplelog::*;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// path of config file
    #[arg(short, long, default_value = "../etc/bench.yaml")]
    config: String,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn load_config(config_path: &str) -> Result<BenchConfig> {
    let builder = AppConfig::builder()
        .add_source(AppFile::with_name(config_path))
        .add_source(AppConfig::try_from(&BenchConfig::default()).unwrap());
    let cfg = builder.build()?;
    Ok(cfg.try_deserialize()?)
}

fn main() {
    let args = Args::parse();
    let cfg = load_config(&args.config).unwrap();

    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::from_str(&cfg.log.log_level).unwrap(),
        Config::default(),
        std::fs::File::create(cfg.log.log_path.clone()).unwrap(),
    )])
    .unwrap();

    info!("config: {:?}", cfg);
    info!("start to run client");
    let client = client::Client::new(cfg);
    client.run();
    info!("finish to run client");
}
