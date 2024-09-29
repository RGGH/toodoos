mod database; 

use database::Counter; 
use iced::widget::{button, column, text, text_input, Checkbox, Column};
use iced::{ Center, Fill, Theme};
use iced::widget::checkbox;
use tokio;

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    InputChanged(String),
    Reset,
    SaveToDatabase,
    CheckboxToggled(bool),
}

impl Counter {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::CheckboxToggled(is_checked) => {
                self.is_checked = is_checked;
            }
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
                let is_checked = self.is_checked.clone();

                tokio::spawn(async move {
                    let counter = Counter { value, input_value,is_checked};
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

        let checkbox:Checkbox<_> = checkbox("Toggle me!", self.is_checked)
            .on_toggle(Message::CheckboxToggled);

        let content = column![
            text_input,
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement),
            button("Reset").on_press(Message::Reset),
            button("Save").on_press(Message::SaveToDatabase),
            checkbox
        ]
        .width(Fill)
        .spacing(10)
        .padding(20)
        .align_x(Center);

        content
    }
}

fn theme(_: &Counter) -> Theme {
    Theme::TokyoNight
}

pub fn main() -> iced::Result {
    // Create a Tokio runtime
    let runtime = tokio::runtime::Runtime::new().unwrap();

    // Run the application in the Tokio runtime
    runtime.block_on(async {
        iced::application("Learning Rust", Counter::update, Counter::view)
            .theme(theme)
            .run() // Run the application
    })
}
