use iced::widget::{button, column, container, row, scrollable, text, Column, Scrollable, Text};
use iced::{Alignment, Element, Renderer, Sandbox, Settings};
use iced_native::Length;
use serde::{Deserialize, Serialize};

pub fn main() -> iced::Result {
    ListPerson::run(Settings::default())
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Person {
    pub name: String,
    pub age: i32,
}

impl Person {
    pub(crate) fn view(&self) -> Element<Message> {
        row![text(self.name.clone()), text(self.age.to_string())]
            .spacing(8)
            .into()
    }
}

struct ListPerson {
    persons: Vec<Person>,
}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl Sandbox for ListPerson {
    type Message = Message;

    fn new() -> Self {
        Self {
            persons: vec![
                Person {
                    name: "Alice".to_string(),
                    age: 32,
                },
                Person {
                    name: "Bob".to_string(),
                    age: 42,
                },
            ],
        }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {}
    }

    fn view(&self) -> Element<Message> {
        let list = self
            .persons
            .iter()
            .map(|p| p.view())
            .collect::<Vec<Element<Message>>>();

        let col = Column::with_children(list);

        let scrollable = scrollable(container(col).width(Length::Fill));

        container(scrollable).into()
    }
}
