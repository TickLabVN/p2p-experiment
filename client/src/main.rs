use std::net::UdpSocket;

const LOCAL_ADDR: &str = "0.0.0.0:12345";
const STUN_SERVER_ADDR: &str = "14.225.192.183:8888";

fn main() -> std::io::Result<()> {
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
    let socket = UdpSocket::bind(LOCAL_ADDR)?;

    // Send the message to the server
    socket.send_to(message, STUN_SERVER_ADDR)?;
    println!("Sent: {}", String::from_utf8_lossy(message));

    // Receive response from server
    let mut buf = [0; 1024];

    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let response = &buf[..amt];
        let msg = String::from_utf8_lossy(response);
        if msg.is_empty() {
            println!("stun server acknowledged");
            continue;
        }
        if msg == "hello" {
            println!("Received 'hello' from peer {}", src);
            continue;
        }

        println!(
            "Received: {} from {}",
            msg,
            src
        );
        let target_addr = msg.to_string();
        println!("Sending 'hello' to target address: {}", target_addr);
        socket.send_to(b"hello", target_addr)?;
    }
}

