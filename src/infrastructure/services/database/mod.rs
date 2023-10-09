use async_std::sync::Mutex;
use async_std::task;
use mongodb::{Client, Database};
use once_cell::sync::Lazy;

pub struct MongoDatabase {
    pub db: Database,
}

impl MongoDatabase {
    pub fn init() -> Self {
        let client = task::block_on(Client::with_uri_str(String::from(
            "mongodb://localhost:27017",
        )))
        .unwrap();

        Self {
            db: client.database("rustDB"),
        }
    }
}

pub static DB: Lazy<Mutex<MongoDatabase>> = Lazy::new(|| Mutex::new(MongoDatabase::init()));
