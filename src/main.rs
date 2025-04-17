:quick-xmluse iced::{Element, Settings, run};
use iced::widget::{button, column, text, Column};

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.value += 1,
        Message::Decrement => counter.value -= 1,
    }
}

fn view(counter: &Counter) -> Element<Message> {

    column![
        text("Shit!"),
        button("+").on_press(Message::Increment),
        text(counter.value),
        button("-").on_press(Message::Decrement),
    ]
        .into()
}

fn main() -> iced::Result {
    run("Fuck given", update, view)
}
