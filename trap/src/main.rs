pub mod database;
pub mod thread;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    //thread,
};
use thread::ThreadPool;
use database::DBStore;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let pool = ThreadPool::new(4);
    let database = DBStore::new();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) -> Vec<String>{
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
    return http_request;
}