pub mod env;

use env::AppEnv;
use std::{io::Result, net::UdpSocket, thread};

fn listen_udp_socket(port: u16) -> Result<()> {
    let mut buf = [0u8; 1024];
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", port))?;
    println!("Listening on port {}", port);

    loop {
        let (_, src) = socket.recv_from(&mut buf)?;
        let response = src.to_string();
        socket.send_to(response.as_bytes(), src)?;
    }
}

fn main() {
    let app_env = AppEnv::load();

    let ports = app_env.ports;
    if ports.is_empty() {
        eprintln!("No ports specified in the environment. Exiting.");
        return;
    }

    let mut handles = Vec::new();
    for port in ports {
        let handle = thread::spawn(move || {
            if let Err(e) = listen_udp_socket(port) {
                eprintln!("Handler on port {} failed: {}", port, e);
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}
