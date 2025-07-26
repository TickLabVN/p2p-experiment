use std::{io::Result, net::UdpSocket, str};

fn main() -> Result<()> {
    const PORT: u16 = 8888; // Define the port to listen on
    // Bind to an address and port
    let socket = UdpSocket::bind(format!("127.0.0.1:{}", PORT))?;
    println!("UDP server listening on 127.0.0.1:{}", PORT);

    let mut buf = [0u8; 1024]; // buffer to store incoming data

    loop {
        // Receive data from any client
        let (amt, src) = socket.recv_from(&mut buf)?;
        let msg = str::from_utf8(&buf[..amt]).unwrap_or("[Invalid UTF-8]");
        println!("Received from {}: {}", src, msg);

        // Optional: Send response back
        let response = b"Message received!";
        socket.send_to(response, &src)?;
    }
}
