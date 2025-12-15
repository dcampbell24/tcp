use std::{
    io::{BufRead, BufReader}, net::TcpStream, sync::mpsc::channel, thread::{self, sleep}, time::Duration
};

const ADDRESS: &str = "hnefatafl.org:49152";

fn main() {
    let (tx, rx) = channel();

    thread::spawn(move || {
        loop {
            let message: &str = rx.recv().unwrap();
            let message_trim = message.trim();

            if message_trim == "start_tcp" {
                println!("Connecting to server...");
                let tcp_stream = TcpStream::connect(ADDRESS).unwrap();
                println!("Connected.");

                let mut reader = BufReader::new(tcp_stream.try_clone().unwrap());

                thread::spawn(move || {
                    let mut buf = String::new();
                    let count = reader.read_line(&mut buf).unwrap();
                    println!("{count}: {buf}");
                    buf.clear();
                });

                loop {
                    let message = rx.recv().unwrap();
                    let message_trim = message.trim();

                    if message_trim == "stop_tcp" {
                        println!("Disconnected.");
                        return;
                    }
                }
            }
        }
    });

    tx.send("start_tcp").unwrap();
    sleep(Duration::from_secs(3));

    tx.send("stop_tcp").unwrap();

    sleep(Duration::from_secs(1));
}
