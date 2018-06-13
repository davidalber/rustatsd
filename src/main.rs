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

struct BufferWindowManager {
    buf: [[u8; BUFFER_SIZE]; BUFFER_WINDOW_COUNT],
    buf_ready: [bool; BUFFER_SIZE],
    buf_window: usize,
}

impl BufferWindowManager {
    fn new() -> BufferWindowManager {
        BufferWindowManager {
            buf: [[0; BUFFER_SIZE]; BUFFER_WINDOW_COUNT],
            buf_ready: [true; BUFFER_SIZE],
            buf_window: BUFFER_WINDOW_COUNT-1,
        }
    }

    fn get_next(&mut self) -> usize {
        let ret = self.buf_window;
        self.buf_window = (self.buf_window + 1) % BUFFER_WINDOW_COUNT;
        self.buf_ready[ret] = false;
        ret
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
    let mut buf_window_manager = BufferWindowManager::new();

    loop {
        let buf_window = buf_window_manager.get_next();
        match socket.recv_from(&mut buf_window_manager.buf[buf_window]) {
            Ok((amt, _src)) => {
                println!("{:?}", String::from_utf8_lossy(&buf_window_manager.buf[buf_window][..amt]));
            },
            Err(_err) => (),
        };
    }
}
