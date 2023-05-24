use std::fmt;
use iced::{
    widget::{button, column, text},
    {Alignment, Element, Sandbox, Settings},
};

pub fn run() -> iced::Result {
    Player::run(Settings::default())
}

struct Player {
    state: State,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum State {
    Paused,
    Playing,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    PlayPause,
}

impl Sandbox for Player {
    type Message = Message;

    fn new() -> Self {
        Self { state: State::Paused }
    }

    fn title(&self) -> String {
        String::from("rain")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::PlayPause => {
                if self.state == State::Paused {
                    self.state = State::Playing;
                } else {
                    self.state = State::Paused;
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        column![
            button("⏯").on_press(Message::PlayPause),
            text(self.state).size(50),
        ]
        .align_items(Alignment::Center)
        .into()
    }
}