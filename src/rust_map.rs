use std::net::TcpStream;
use std::io::prelude::*;


pub fn rust_scan(ip: &str) {

    for number in 0..65535 {
        let port = number.to_string();
        rust_scan_tcp(ip, port);
    }

}


fn rust_scan_tcp(ip: &str, port: String){
    println!("Connecting to {}:{}", ip, port);
    let scan_target = format!("{}:{}", ip, port);

    //this works if the port is closed or open but not filtered
    if let Ok(_stream) = TcpStream::connect(scan_target) {
        println!("{}:{} is open", ip, port);
    } else {
        println!("Unable to connect");
    }
}