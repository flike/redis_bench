use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchConfig {
    pub redis_addr: String,
    pub prometheus_addr: String,
    pub thread_num: u32,
    pub query_type: String,
    pub read_opt: ReadOpt,
    pub write_opt: WriteOpt,
    pub log: LogOpt,
}

impl Default for BenchConfig {
    fn default() -> Self {
        BenchConfig {
            redis_addr: "127.0.0.1:6379".to_string(),
            prometheus_addr: "127.0.0.1:9800".to_string(),
            thread_num: 1,
            query_type: "read".to_string(),
            read_opt: ReadOpt::default(),
            write_opt: WriteOpt::default(),
            log: LogOpt::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReadOpt {
    pub key_num: u32,
    pub key_prefix: String,
}

impl Default for ReadOpt {
    fn default() -> Self {
        ReadOpt {
            key_num: 1000,
            key_prefix: "read_key".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogOpt {
    pub log_path: String,
    pub log_level: String,
}

impl Default for LogOpt {
    fn default() -> Self {
        LogOpt {
            log_path: "./bench.log".to_string(),
            log_level: "info".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WriteOpt {
    pub key_num: u32,
    pub key_prefix: String,
    pub expire_time: u64,
}

impl Default for WriteOpt {
    fn default() -> Self {
        WriteOpt {
            key_num: 1000,
            key_prefix: "write_key".to_string(),
            expire_time: 60,
        }
    }
}
