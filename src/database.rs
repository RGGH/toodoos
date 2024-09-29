use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;
use surrealkv::{IsolationLevel, Options, Store};

#[derive(Default, Serialize)]
pub struct Counter {
    pub value: i64,
    pub input_value: String,
    pub is_checked: bool
}

impl Counter {
    pub async fn save_to_database(&self) -> Result<(), Box<dyn Error>> {
        let options = Options {
            dir: PathBuf::from("."),
            isolation_level: IsolationLevel::SerializableSnapshotIsolation,
            ..Default::default()
        };

        let db = Store::new(options)?;
        let mut conn = db.begin()?;

        // Create a unique key based on the current timestamp
        let key = format!("counter:{}:{}", self.value, Utc::now().timestamp());

        // Convert the Counter instance to a JSON string
        let counter_json = serde_json::to_string(self)?;

        // Save the data to the database
        let _ = conn.set(key.as_bytes(), counter_json.as_bytes())?;

        // Retrieve the record we just added
        let res = conn.get(key.as_bytes());
        match res {
            Ok(Some(value_bytes)) => {
                // Convert the retrieved bytes to a string
                if let Ok(retrieved_value) = String::from_utf8(value_bytes) {
                    println!("Record added: {}", retrieved_value);
                } else {
                    println!("Error converting bytes to string");
                }
            }
            Ok(None) => println!("Key not found"),
            Err(e) => println!("Error retrieving key: {:?}", e),
        }

        Ok(())
    }
}
