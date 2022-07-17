mod rustMap;


fn main() {
    let ipaddress = std::env::args().nth(1).expect("no pattern given");
    let port = std::env::args().nth(2).expect("no path given");
    //Convert ipaddress to str from Stringl
    rustMap::rust_scan(&ipaddress, &port);
}