mod rust_map;


fn main() {
    let operation = std::env::args().nth(1).expect("No operation choosen");
    if operation == "scan" {
        call_rust_scan()
}
}

fn call_rust_scan() {
    let ipaddress = std::env::args().nth(2).expect("No ip given");
    let port = std::env::args().nth(3).expect("No port given");
    rust_map::rust_scan(&ipaddress, &port);
}
