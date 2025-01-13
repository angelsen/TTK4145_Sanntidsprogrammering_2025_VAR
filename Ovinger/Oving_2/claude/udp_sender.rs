use std::io::{self, BufRead};
use std::net::UdpSocket;

const SERVER_IP: &str = "127.0.0.1"; // Replace with the actual server IP
const PORT: u16 = 20005; // 20000 + 5 (replace 5 with your workspace number)

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?; // Bind to any available port
    let server_addr = format!("{}:{}", SERVER_IP, PORT);

    println!("Sending messages to {}", server_addr);

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
        print!("Enter message to send (or 'quit' to exit): ");
        io::Write::flush(&mut io::stdout())?;

        if let Some(line) = lines.next() {
            let message = line?;
            if message == "quit" {
                break;
            }

            socket.send_to(message.as_bytes(), &server_addr)?;
            println!("Message sent.");
        }
    }

    Ok(())
}