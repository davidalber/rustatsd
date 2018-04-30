use std::net::UdpSocket;
use std::process;

fn main() {
    let socket = match UdpSocket::bind("127.0.0.1:6000") {
        Ok(socket) => socket,
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
