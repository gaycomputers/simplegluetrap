extern crate r2d2;
//pub mod database;
pub mod thread;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    //thread,
};
use r2d2_sqlite::SqliteConnectionManager;
use thread::ThreadPool;
//use database::DBStore;

use sha2::{Sha256, Digest};
use rusqlite::params;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let pool = ThreadPool::new(4);

    //database stuff
    //let database = DBStore::new();
    let manager = SqliteConnectionManager::file("requests.db");
    let sqlpool = r2d2::Pool::new(manager).unwrap();
    sqlpool.get()
        .unwrap()
        .execute("create table if not exists requests (
            uuid text primary key,
            request text not null)", params![])
        .unwrap();

    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let sqlpool = sqlpool.clone();
        
        pool.execute(move || {
            let mut hasher = Sha256::new();
            let request = handle_connection(stream);
            let s = request.concat();
            hasher.update(&s);
            let key = format!("{:X}", hasher.finalize());
            let conn = sqlpool.get().unwrap();
            conn.execute("REPLACE INTO requests (uuid, request) values (?1, ?2)", [key, s]).unwrap();
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