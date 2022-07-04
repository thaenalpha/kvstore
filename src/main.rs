use std::{collections::HashMap, iter};
use std::str::SplitN;

fn main() {
    let mut arguments: iter::Skip<std::env::Args> = std::env::args().skip(1);
    let key: String = arguments.next().expect("key was not there");
    let value: String = arguments.next().unwrap();
    println!("The key is '{}' and the vaule is '{}'", key, value);
    let contents: String = format!("{}={}", key, value);
    std::fs::write("kv.db", contents).unwrap();

    let database: Database = Database::new().expect("Could not create database");
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map: HashMap<String, String> = HashMap::new();
        let contents: String = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let mut chunks: SplitN<char> = line.splitn(2, '=');
            let key: &str = chunks.next().expect("key was not there");
            let value: &str = chunks.next().expect("value was not there");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database { map: map })
    }
}

// TODO: create PathBuf for kv.db, check for exist and create if not exist.
// TODO: create a function to write to kv.db.
