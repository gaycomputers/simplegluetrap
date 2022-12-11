//goal here is a handler that receives data and writes it to memory, then every x receiving writes everything to sqlite on disk with a spawned thread
use std::{
    //thread,
    collections::HashMap,
};
//use sha2::digest::crypto_common::Key;
use sha2::{Sha256, Digest};
//use rusqlite::NO_PARAMS;
use rusqlite::{Connection};
//use hex_literal::hex;

pub struct DBStore {
    dict: HashMap<String,String>,
    counter: i8,
    sql: Connection,
}
impl DBStore {
    pub fn new() -> DBStore {
        let dict = HashMap::new();
        let counter = 0;
        let sql = Connection::open("requests.db").unwrap();
        sql.execute(
            "create table if not exists requests (
                 id integer primary key,
                 uuid text not null,
                 request text not null
             )",
            [],
        ).unwrap();
        DBStore { dict, counter, sql}
    }
    pub fn add(&mut self, request: Vec<String>){
        let s = request.concat();
        let mut hasher = Sha256::new();
        hasher.update(&s);
        let key = format!("{:X}", hasher.finalize());
        self.dict.insert(key.clone(), s);
        if self.counter < 100 {
            self.counter += 1;
        }
        else {
            self.counter = 0;
            for (key, request) in &self.dict{
                self.sql.execute(
                    "INSERT INTO requests (uuid, request) values (?1, ?2)",
                    &[key, request],
                ).unwrap();
            }
        }
    }
}