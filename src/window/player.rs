use std::fmt;
use iced::{
    widget::{button, column, text, container},
    {Alignment, Element, Sandbox, Settings},
};

pub fn run() -> iced::Result {
    App::run(Settings::default())
}

struct App {
    state: PlayerState,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum PlayerState {
    Paused,
    Playing,
}

impl fmt::Display for PlayerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

fn change_player_state(state: &mut PlayerState) {
    if *state == PlayerState::Paused {
        *state = PlayerState::Playing;
    } else {
        *state = PlayerState::Paused;
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    PlayPause,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self { state: PlayerState::Paused }
    }

    fn title(&self) -> String {
        String::from("rain")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::PlayPause => { change_player_state(&mut self.state); }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let change_button = column![
            button("change state").on_press(Message::PlayPause),
            text(self.state).size(50),
        ]
            .align_items(Alignment::Center);

        container(change_button)
            .center_x()
            .center_y()
            .into()
    }
}