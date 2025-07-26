use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    const PORT: u16 = 8888;
    // Create a UDP socket (bind to any available local port)
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // Server address to send the packet to
    let server_addr = format!("127.0.0.1:{}", PORT);

    let message = b"Hello from client!";

    // Send the message to the server
    socket.send_to(message, server_addr)?;
    println!("Sent: {}", String::from_utf8_lossy(message));
    Ok(())
}
