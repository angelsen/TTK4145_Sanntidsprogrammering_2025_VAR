use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:30000")?;
    
    println!("Listening on port 30000...");

    let mut buf = [0; 1024];
    let (amt, src) = socket.recv_from(&mut buf)?;

    let received = String::from_utf8_lossy(&buf[..amt]);
    println!("Received IP from {}: {}", src, received);

    Ok(())
}