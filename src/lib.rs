extern crate config;
extern crate seahash;

use seahash::hash;
use std::net::UdpSocket;
use std::process;
use std::sync::mpsc::{channel, Sender};
use std::thread;

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

pub struct MetricIngester {
    config: Config,
    socket: UdpSocket,
    worker_senders: Vec<Sender<Option<String>>>,
}

impl MetricIngester {
    pub fn new() -> MetricIngester {
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
        let mut worker_senders = Vec::new();
        for i in 0..config.worker_count {
            let (tx, rx) = channel();
            worker_senders.push(tx);
            thread::spawn(move || {
                loop {
                    match rx.recv().unwrap() {
                        Some(metric) => println!("[In worker {}] {}", i, metric),
                        None => break,
                    };
                }
            });
        }

        MetricIngester { config, socket, worker_senders }
    }

    fn get_stat_worker(&self, stat_name: &str) -> usize {
        (hash(stat_name.as_bytes()) % self.config.worker_count as u64) as usize
    }

    fn process_stats(&self, buf: &[u8]) {
        let buf_cow = String::from_utf8_lossy(buf);
        for stat in buf_cow.split("\n") {
            if let Some(stat_name) = stat.split(":").nth(0) {
                let worker = self.get_stat_worker(stat_name);
                println!("[ {} ] {}", worker, stat);
                self.worker_senders[worker].send(Some(stat.to_string())).unwrap();
            }
        }
    }

    pub fn run(&self) {
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
