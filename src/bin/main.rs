use clap::Parser;
use log::*;
use redis_bench::client;
use redis_bench::config;
use simplelog::*;
use std::fs::File;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// path of config file
    #[arg(short, long, default_value = "../etc/bench.yaml")]
    config: String,
}

fn main() {
    let args = Args::parse();
    let cfg = config::load_config(&args.config);

    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::from_str(&cfg.log.log_level).unwrap(),
        Config::default(),
        File::create(cfg.log.log_path.clone()).unwrap(),
    )])
    .unwrap();

    info!("config: {:?}", cfg);
    info!("start to run client");
    let client = client::Client::new(cfg);
    client.run();
    info!("finish to run client");
}
