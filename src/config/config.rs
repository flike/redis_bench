use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct BenchConfig {
    pub redis_addr: String,
    pub prometheus_addr: String,
    pub thread_num: u32,
    pub query_type: String,
    pub read_opt: ReadOpt,
    pub write_opt: WriteOpt,
    pub log: LogOpt,
}
#[derive(Debug, Deserialize, Clone)]
pub struct ReadOpt {
    pub key_num: u32,
    pub key_prefix: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LogOpt {
    pub log_path: String,
    pub log_level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WriteOpt {
    pub key_num: u32,
    pub key_prefix: String,
    pub expire_time: u64,
}

pub fn load_config(config_path: &str) -> Box<BenchConfig> {
    let config_str = std::fs::read_to_string(config_path).unwrap();
    let config: BenchConfig = serde_yaml::from_str(&config_str).unwrap();
    Box::new(config)
}
