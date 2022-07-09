use std::net::TcpStream;
use std::prelude::*;

pub fn rust_scan(ip: &str, port: &str) {
    println!("Connecting to {}:{}", ip, port);
    let scan_target = format!("{}:{}", ip, port);

    if let Ok(Stream) = TcpStream::connect(scan_target) {
        println!("{}:{} is open", ip, port);
    } else {
        println!("{}:{} is either closed or filtered", ip, port);
    }
    return();
}