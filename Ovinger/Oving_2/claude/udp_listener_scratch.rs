use std::mem;
use std::str;
use libc::{c_void, socklen_t, AF_INET, INADDR_ANY, SOCK_DGRAM, SOL_SOCKET, SO_REUSEADDR};

const PORT: u16 = 30000;
const BUFFER_SIZE: usize = 1024;

#[repr(C)]
struct sockaddr_in {
    sin_family: libc::sa_family_t,
    sin_port: u16,
    sin_addr: u32,
    sin_zero: [u8; 8],
}

fn main() -> Result<(), String> {
    unsafe {
        // Create socket
        let socket = libc::socket(AF_INET, SOCK_DGRAM, 0);
        if socket < 0 {
            return Err("Failed to create socket".to_string());
        }

        // Set SO_REUSEADDR option
        let opt: libc::c_int = 1;
        if libc::setsockopt(socket, SOL_SOCKET, SO_REUSEADDR, &opt as *const _ as *const c_void, mem::size_of_val(&opt) as socklen_t) < 0 {
            return Err("Failed to set SO_REUSEADDR".to_string());
        }

        // Prepare the sockaddr_in structure
        let addr = sockaddr_in {
            sin_family: AF_INET as libc::sa_family_t,
            sin_port: PORT.to_be(),
            sin_addr: INADDR_ANY,
            sin_zero: [0; 8],
        };

        // Bind
        if libc::bind(socket, &addr as *const _ as *const libc::sockaddr, mem::size_of_val(&addr) as socklen_t) < 0 {
            return Err("Bind failed".to_string());
        }

        println!("Listening on port {}...", PORT);

        let mut buffer = [0u8; BUFFER_SIZE];
        let mut src_addr: sockaddr_in = mem::zeroed();
        let mut src_len = mem::size_of::<sockaddr_in>() as socklen_t;

        // Receive data
        let received = libc::recvfrom(socket, 
                                      buffer.as_mut_ptr() as *mut c_void, 
                                      BUFFER_SIZE, 
                                      0, 
                                      &mut src_addr as *mut _ as *mut libc::sockaddr, 
                                      &mut src_len);

        if received < 0 {
            return Err("Failed to receive data".to_string());
        }

        let received_str = str::from_utf8(&buffer[..received as usize])
            .map_err(|e| e.to_string())?;

        println!("Received: {}", received_str);

        libc::close(socket);
    }

    Ok(())
}