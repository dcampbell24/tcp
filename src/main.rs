use std::net::TcpStream;

const ADDRESS: &str = "hnefatafl.org:49152";

fn main() {
    println!("Connecting to server...");
    TcpStream::connect(ADDRESS).unwrap();
    println!("Connected.");
}
