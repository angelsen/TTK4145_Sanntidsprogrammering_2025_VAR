use std::io::{self, Read, Write};
use std::net::{TcpStream, TcpListener};

const SERVER_IP: &str = "127.0.0.1"; // Replace with the IP obtained from UDP
const FIXED_SIZE_PORT: u16 = 34933;
const NULL_TERM_PORT: u16 = 33546;

fn main() -> io::Result<()> {
    // Connect to the server for fixed-size messages
    let mut fixed_stream = TcpStream::connect((SERVER_IP, FIXED_SIZE_PORT))?;
    println!("Connected to server (fixed-size messages)");

    // Receive welcome message
    let mut buffer = [0; 1024];
    let bytes_read = fixed_stream.read(&mut buffer)?;
    println!("Welcome message: {}", String::from_utf8_lossy(&buffer[..bytes_read]));

    // Send and receive a fixed-size message
    fixed_stream.write_all(b"Hello, Server!")?;
    let bytes_read = fixed_stream.read(&mut buffer)?;
    println!("Received (fixed-size): {}", String::from_utf8_lossy(&buffer[..bytes_read]));

    // Connect to the server for null-terminated messages
    let mut null_stream = TcpStream::connect((SERVER_IP, NULL_TERM_PORT))?;
    println!("Connected to server (null-terminated messages)");

    // Receive welcome message
    let mut buffer = Vec::new();
    null_stream.read_until(b'\0', &mut buffer)?;
    println!("Welcome message: {}", String::from_utf8_lossy(&buffer));

    // Send and receive a null-terminated message
    null_stream.write_all(b"Hello, Server!\0")?;
    buffer.clear();
    null_stream.read_until(b'\0', &mut buffer)?;
    println!("Received (null-terminated): {}", String::from_utf8_lossy(&buffer));

    // Set up a listener for the server to connect back
    let listener = TcpListener::bind("0.0.0.0:0")?;
    let local_addr = listener.local_addr()?;

    // Send the "Connect to" message
    let connect_msg = format!("Connect to: {}:{}\0", local_addr.ip(), local_addr.port());
    null_stream.write_all(connect_msg.as_bytes())?;
    println!("Sent connect request: {}", connect_msg);

    // Accept the incoming connection from the server
    let (mut stream, _) = listener.accept()?;
    println!("Server connected back");

    // Receive message from the server on the new connection
    buffer.clear();
    stream.read_until(b'\0', &mut buffer)?;
    println!("Received on new connection: {}", String::from_utf8_lossy(&buffer));

    Ok(())
}