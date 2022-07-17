use std::net::TcpStream;

pub fn rust_scan(ip: &str, port: &str) {
    println!("Connecting to {}:{}", ip, port);
    let scan_target = format!("{}:{}", ip, port);

    

    //this works if the port is closed or open but not filtered
    if let Ok(_stream) = TcpStream::connect(scan_target) {
        println!("{}:{} is open", ip, port);
    } else {
        println!("{}:{} is closed", ip, port);
    }
    return();
}