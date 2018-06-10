extern crate config;

use std::net::UdpSocket;
use std::process;

struct Config {
    port: u32,
    flush_interval: u32,
}

impl Config {
    fn new(conf: config::Config) -> Config {
        let port = conf.get_int("port").unwrap_or(8125) as u32;
        let flush_interval = conf.get_int("flushInterval").unwrap_or(102) as u32;
        Config { port, flush_interval }
    }
}

fn load_config() -> Config {
    let mut config = config::Config::default();
    config
        .merge(config::File::with_name("Config")).unwrap();
    Config::new(config)
}

fn main() {
    let config = load_config();
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

    let mut buf = [0; 576];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, _src)) => {
                println!("{:?}", String::from_utf8_lossy(&buf[..amt]));
            },
            Err(_err) => (),
        };
    }
}
