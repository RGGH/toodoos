#![allow(unused)]
use iced::widget::{button, center, column, text, text_input, Column};
use iced::{Center, Element, Fill, Theme};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc; // Add this for Arc
use surrealkv::{IsolationLevel, Options, Store};
use tokio;

pub fn main() -> iced::Result {
    let runtime = tokio::runtime::Runtime::new().unwrap(); // Create a Tokio runtime
    runtime.block_on(async { iced::run("Learning Rust", Counter::update, Counter::view) })
}

#[derive(Default, Serialize)]
struct Counter {
    value: i64,
    input_value: String, // Add input_value here
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    InputChanged(String), // Keep this for handling input changes
    Reset,
    SaveToDatabase,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
            Message::InputChanged(input) => {
                self.input_value = input; // Update input_value when input changes
            }
            Message::Reset => {
                self.value = 0;
                self.input_value.clear(); // Optionally clear the input when reset
            }
            Message::InputChanged(input) => {
                self.input_value = input;
            }
            Message::SaveToDatabase => {
                // Save data to database asynchronously
                let input_value = self.input_value.clone();
                let value = self.value.clone();

                tokio::spawn(async move {
                    let counter = Counter { value, input_value };
                    if let Err(e) = counter.save_to_database().await {
                        println!("Failed to save to database: {:?}", e);
                    }
                });
            }
        }
    }



    async fn save_to_database(&self) -> Result<(), Box<dyn std::error::Error>> {
        let options = Options {
            dir: PathBuf::from("."),
            isolation_level: IsolationLevel::SerializableSnapshotIsolation,
            ..Default::default()
        };

        let db = Store::new(options)?;
        let mut conn = db.begin()?;

        // SAVE the data to surrealkv
        // Create a unique key based on the current timestamp
        let key = format!("counter:{}:{}", self.value, Utc::now().timestamp());

        // Convert the Counter instance to a JSON string
        let counter_json = serde_json::to_string(self)?;

        // Save the data to the database
        let _ = conn.set(key.as_bytes(), counter_json.as_bytes())?;

        //----------------------- fetch new record!
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

    fn view(&self) -> Column<Message> {
        // Create the text input
        let text_input = text_input("Type something...", &self.input_value)
            .on_input(Message::InputChanged)
            .padding(10)
            .size(20);

        let content = column![
            text_input, // Add the text input to the column
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement),
            button("Reset").on_press(Message::Reset),
            button("Save").on_press(Message::SaveToDatabase)
        ]
        .width(Fill)
        //.max_width(500)
        .spacing(10)
        .padding(20)
        .align_x(Center);

        content
    }
}
