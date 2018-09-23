extern crate config;
extern crate seahash;

use std::net::UdpSocket;
use std::process;
use seahash::hash;

const BUFFER_SIZE: usize = 576;

struct Config {
    port: u32,
    flush_interval: u32,
    worker_count: u8,
}

impl Config {
    fn new(conf: config::Config) -> Config {
        let port = conf.get_int("port").unwrap_or(8125) as u32;
        let flush_interval = conf.get_int("flushInterval").unwrap_or(102) as u32;
        let worker_count = conf.get_int("workerCount").unwrap_or(4) as u8;
        Config { port, flush_interval, worker_count }
    }

    fn load_config() -> Config {
        let mut config = config::Config::default();
        config
            .merge(config::File::with_name("Config")).unwrap();
        Config::new(config)
    }
}

struct MetricIngester {
    config: Config,
    socket: UdpSocket,
}

impl MetricIngester {
    fn new() -> MetricIngester {
        let config = Config::load_config();
        let socket = match UdpSocket::bind(format!("127.0.0.1:{}", config.port)) {
            Ok(socket) => {
                println!("Listening on port {}...", config.port);
                socket
            },
            Err(_err) => {
                eprintln!("Could not bind to socket.");
                process::exit(1);
            },
        };

        MetricIngester { config, socket }
    }

    fn get_stat_worker(&self, stat_name: &str) -> u8 {
        (hash(stat_name.as_bytes()) % self.config.worker_count as u64) as u8
    }

    fn process_stats(&self, buf: &[u8]) {
        let buf_cow = String::from_utf8_lossy(buf);
        for stat in buf_cow.split("\n") {
            if let Some(stat_name) = stat.split(":").nth(0) {
                println!("[ {} ] {:?}", self.get_stat_worker(stat_name), stat);
            }
        }
    }

    fn run(&self) {
        let mut buf = [0; BUFFER_SIZE];
        loop {
            match self.socket.recv_from(&mut buf) {
                Ok((amt, _src)) => {
                    let buf = &buf[..amt];
                    self.process_stats(buf);
                },
                Err(_err) => (),
            };
        }
    }
}

fn main() {
    let metric_ingester = MetricIngester::new();
    metric_ingester.run();
}
