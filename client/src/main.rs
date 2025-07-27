use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    const SERVER_PORT: u16 = 8888;

    // Get command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <message>", args[0]);
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Expected a message to send",
        ));
    }
    let message = args[1].as_bytes();

    // Create a UDP socket (bind to any available local port)
    let socket = UdpSocket::bind("0.0.0.0:12345")?;

    // Server address to send the packet to
    let server_addr = format!("127.0.0.1:{}", SERVER_PORT);

    // Send the message to the server
    socket.send_to(message, server_addr)?;
    println!("Sent: {}", String::from_utf8_lossy(message));

    // Receive response from server
    let mut buf = [0; 1024];
    let (amt, src) = socket.recv_from(&mut buf)?;
    let response = &buf[..amt];
    println!(
        "Received: {} from {}",
        String::from_utf8_lossy(response),
        src
    );
    Ok(())
}
