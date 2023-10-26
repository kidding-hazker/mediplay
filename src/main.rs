pub mod connection;
pub mod request;
pub mod response;
pub mod database;

pub use connection::connect;
pub use request::Request;
pub use response::Response;

use std::net::TcpListener;
use std::thread::{self, JoinHandle};

fn main() {
    let server = TcpListener::bind("127.0.0.1:8080").unwrap();
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for stream in server.incoming() {
        match stream {
            Ok(mut stream) => {
                handles.push(thread::spawn(move || {
                    connect(&mut stream);
                }));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    while handles.len() > 0 {
        let handle = handles.remove(0);

        handle.join().unwrap();
    }
}
