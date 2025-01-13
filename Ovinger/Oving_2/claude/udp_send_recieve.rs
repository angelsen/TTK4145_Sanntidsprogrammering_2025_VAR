use std::net::UdpSocket;
use std::io::{self, BufRead};
use std::str;

const SERVER_IP: &str = "127.0.0.1"; // Replace with the actual server IP
const SERVER_PORT: u16 = 20005; // 20000 + 5 (replace 5 with your workspace number)
const BUFFER_SIZE: usize = 1024;

fn main() -> io::Result<()> {
    // Bind to a local address
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    println!("Local address: {}", socket.local_addr()?);

    // Set the destination address
    let server_addr = format!("{}:{}", SERVER_IP, SERVER_PORT);
    println!("Server address: {}", server_addr);

    // Create a separate thread for receiving messages
    let recv_socket = socket.try_clone()?;
    std::thread::spawn(move || {
        let mut buffer = [0u8; BUFFER_SIZE];
        loop {
            match recv_socket.recv_from(&mut buffer) {
                Ok((size, src)) => {
                    if let Ok(message) = str::from_utf8(&buffer[..size]) {
                        println!("Received from {}: {}", src, message);
                    }
                },
                Err(e) => eprintln!("Error receiving: {}", e),
            }
        }
    });

    // Main thread for sending messages
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
        print!("Enter message to send (or 'quit' to exit): ");
        io::Write::flush(&mut io::stdout())?;

        if let Some(Ok(line)) = lines.next() {
            if line == "quit" {
                break;
            }

            socket.send_to(line.as_bytes(), &server_addr)?;
            println!("Message sent.");
        }
    }

    Ok(())
}