use std::{
    net::TcpStream,
    sync::mpsc::channel,
    thread::{self, sleep},
    time::Duration,
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
                TcpStream::connect(ADDRESS).unwrap();
                println!("Connected.");

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
