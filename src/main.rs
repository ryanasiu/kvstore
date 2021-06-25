use std::collections::HashMap;

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap();
    let val = args.next().unwrap();
    let out = format!("{}\t{}\n", key, val);
    println!("{}->{}", key, val);
    std::fs::write("kv.db", out).unwrap();

    let db = Database::new().expect("Database::new crashed");
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key").to_owned();
            let val = chunks.next().expect("No val").to_owned();
            map.insert(key, val);
        }
        Ok(Database { map: map })
    }
}
