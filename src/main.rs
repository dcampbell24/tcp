use std::{
    net::{Shutdown, TcpStream},
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
                let tcp_stream = TcpStream::connect(ADDRESS).unwrap();
                println!("Connected.");

                loop {
                    let message = rx.recv().unwrap();
                    let message_trim = message.trim();

                    if message_trim == "stop_tcp" {
                        tcp_stream
                            .shutdown(Shutdown::Both)
                            .expect("shutdown call failed");

                        return;
                    }
                }
            }
        }
    });

    tx.send("start_tcp").unwrap();
    tx.send("stop_tcp").unwrap();

    sleep(Duration::from_secs(1));
}
