extern crate config;

use std::net::UdpSocket;
use std::process;

const BUFFER_SIZE: usize = 576;
const BUFFER_WINDOW_COUNT: usize = 5;

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

fn get_next_buf_window(buf_window: usize) -> usize {
    (buf_window + 1) % BUFFER_WINDOW_COUNT
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

    let mut buf: [[u8; BUFFER_SIZE]; BUFFER_WINDOW_COUNT] = [[0; BUFFER_SIZE]; BUFFER_WINDOW_COUNT];
    let mut buf_window = 0;
    loop {
        match socket.recv_from(&mut buf[buf_window]) {
            Ok((amt, _src)) => {
                println!("{:?}", String::from_utf8_lossy(&buf[buf_window][..amt]));
                buf_window = get_next_buf_window(buf_window);
            },
            Err(_err) => (),
        };
    }
}
