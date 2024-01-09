use std::net::IpAddr;
use std::net::UdpSocket;

fn get_local_ip() -> Option<IpAddr> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    socket.local_addr().ok().map(|addr| addr.ip())
}

fn main() {
    if let Some(local_ip) = get_local_ip() {
        println!("Local IP address: {}", local_ip);
    } else {
        println!("Failed to retrieve local IP address");
    }
}
