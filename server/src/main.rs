pub mod env;

use env::AppEnv;
use std::{collections::HashMap, io::Result, net::UdpSocket, str};

struct ConnectCommand {
    pub source: String,
    pub target: String
}
impl ConnectCommand {
    pub fn from(v: &[u8]) -> Option<Self> {
        let parts: Vec<&str> = str::from_utf8(v).ok()?.split('|').collect();
        if parts.len() != 2 {
            return None;
        }
        println!("Received connect command: source={}, target={}", parts[0], parts[1]);
        Some(Self {
            source: parts[0].to_string(),
            target: parts[1].to_string(),
        })
    }
}

fn main() -> Result<()> {
    let app_env = AppEnv::load();
    let addr = format!("127.0.0.1:{}", app_env.port);
    let socket = UdpSocket::bind(&addr)?;

    println!("UDP server listening on {}", &addr);

    let mut buf = [0u8; 1024]; // buffer to store incoming data
    // UserID -> [ip]:[port]
    let mut peers = HashMap::<String, String>::new();

    loop {
        // Receive data from any client
        let (amt, src) = socket.recv_from(&mut buf)?;
        let connect_req = ConnectCommand::from(&buf[..amt]);
        if connect_req.is_none() {
            println!("Received invalid command from {}", src);
            socket.send_to(b"Invalid command", src)?;
            continue;
        }
        let connect_req = connect_req.unwrap();

        if let Some(peer_addr) = peers.remove(&connect_req.target) {
            println!("Found peer addr {}", peer_addr);
            socket.send_to(peer_addr.as_bytes(), src)?;
        } else {
            peers.insert(connect_req.source, src.to_string());
            socket.send_to(b"wait", src)?;
        }
    }
}
