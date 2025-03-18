use iced::widged::text;
struct Counter {
    value: i64,
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
        }
    }
}

enum Message {
    Increment,
    Decrement,
}

fn gui() -> iced::Result {
    iced::run("Espanso", Counter::update, view)
}
fn main() {
    let mut counter = Counter { value: 0 };
    counter.update(Message::Decrement);
    counter.update(Message::Decrement);
    counter.update(Message::Decrement);
    counter.update(Message::Decrement);
    counter.update(Message::Increment);

    println!("Hello, world!");
}
