mod database; // Import the new module

use database::Counter; // Use the Counter struct from the new file
use iced::widget::{button, column, text, text_input, Column};
use iced::{Center, Element, Fill, Theme};
use tokio;

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    InputChanged(String),
    Reset,
    SaveToDatabase,
}

impl Counter {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
            Message::InputChanged(input) => {
                self.input_value = input;
            }
            Message::Reset => {
                self.value = 0;
                self.input_value.clear();
            }
            Message::SaveToDatabase => {
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

    pub fn view(&self) -> Column<Message> {
        let text_input = text_input("Type something...", &self.input_value)
            .on_input(Message::InputChanged)
            .padding(10)
            .size(20);

        let content = column![
            text_input,
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement),
            button("Reset").on_press(Message::Reset),
            button("Save").on_press(Message::SaveToDatabase)
        ]
        .width(Fill)
        .spacing(10)
        .padding(20)
        .align_x(Center);

        content
    }
}

pub fn main() -> iced::Result {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async { iced::run("Learning Rust", Counter::update, Counter::view) })
}

