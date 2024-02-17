use super::config::BenchConfig;
use log::*;
use redis::{Commands, RedisError};
use std::{sync::Arc, thread};

pub struct Client {
    pub cfg: BenchConfig,
    pub redis_client: Arc<redis::Client>,
}

impl Client {
    pub fn new(bench_cfg: BenchConfig) -> Client {
        let redis_addr = format!("redis://{}", bench_cfg.redis_addr);
        let client = redis::Client::open(redis_addr).unwrap();
        Client {
            cfg: bench_cfg,
            redis_client: Arc::new(client),
        }
    }
    pub fn run(&self) {
        if self.cfg.query_type == "read" {
            self.run_read_query();
        } else {
            self.run_write_query();
        }
    }

    pub fn run_read_query(&self) {
        let mut threads = vec![];
        for i in 0..self.cfg.thread_num {
            let read_opt = self.cfg.read_opt.clone();
            let one_thread_query = read_opt.key_num / self.cfg.thread_num;
            let read_client = Arc::clone(&self.redis_client);

            let j = thread::spawn(move || {
                let start = i * one_thread_query;
                let end = (i + 1) * one_thread_query;
                let mut conn = read_client.get_connection().unwrap();
                for j in start..end {
                    let key = format!("{}_{}", read_opt.key_prefix, j);
                    let r: Result<String, RedisError> = conn.get(&key);
                    match r {
                        Ok(_) => {}
                        Err(e) => {
                            error!("read key:{} failed, error:{}", &key, e);
                        }
                    }
                }
                log::info!("read thread {} finished,start:{},end:{}", i, start, end);
            });
            threads.push(j);
        }

        for t in threads {
            t.join().unwrap();
        }
    }

    pub fn run_write_query(&self) {
        let mut threads = vec![];
        for i in 0..self.cfg.thread_num {
            let write_opt = self.cfg.write_opt.clone();
            let one_thread_query = write_opt.key_num / self.cfg.thread_num;
            let read_client = Arc::clone(&self.redis_client);

            let j = thread::spawn(move || {
                let start = i * one_thread_query;
                let end = (i + 1) * one_thread_query;
                let mut conn = read_client.get_connection().unwrap();
                for j in start..end {
                    let key = format!("{}_{}", write_opt.key_prefix, j);
                    let value = format!("value_{}", j);
                    let _: String = conn.set_ex(key, value, write_opt.expire_time).unwrap();
                }
                log::info!("write thread {} finished,start:{},end:{}", i, start, end);
            });
            threads.push(j);
        }

        for t in threads {
            t.join().unwrap();
        }
    }
}
