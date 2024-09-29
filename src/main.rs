#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use iced::widget::{button, text};
use iced::Element;

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

pub fn main() -> iced::Result {
    iced::run("A cool counter", update, view)
}

fn update(counter: &mut u64, message: Message) {
    match message {
        Message::Increment => *counter += 1,
    }
}

fn view(counter: &u64) -> Element<Message> {
    button("").on_press(Message::Increment).into()
}
