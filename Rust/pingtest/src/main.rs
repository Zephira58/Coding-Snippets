use std::net::{IpAddr, TcpStream};
use std::time::Duration;

fn is_server_alive(ip: IpAddr, port: u16, timeout_secs: u64) -> bool {
    if let Ok(_) = TcpStream::connect_timeout(&(ip, port).into(), Duration::from_secs(timeout_secs)) {
        // Connection succeeded, server is alive
        true
    } else {
        // Connection failed, server is not alive or not responding
        false
    }
}

use std::str::FromStr;

fn main() {
    let server_ip = IpAddr::from_str("142.250.70.142").unwrap();
    let server_port = 80; // Example port number
    let timeout_secs = 5;

    let is_alive = is_server_alive(server_ip, server_port, timeout_secs);
    println!("Is server alive? {}", is_alive);
}
