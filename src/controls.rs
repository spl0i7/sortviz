use iced::{Alignment, button, Button, Column, Length, Row};
use iced::button::State;
use iced::Font::Default;
use crate::visualizer;
use crate::visualizer::*;


#[derive(Debug, Clone)]
pub struct Controls {
    state: visualizer::State,
    play_pause: button::State,
    reset: button::State
}

impl Controls {
    pub fn new(state: visualizer::State) -> Self {
        Controls {
            state,
            play_pause: button::State::default(),
            reset: button::State::default()
        }
    }

    pub fn update(&mut self, state: visualizer::State )  {
        self.state = state;
    }

    pub fn view(&mut self) -> Column<Message> {
        let Controls {
            state: _,
            play_pause,
            reset,
        } = self;

        let play_pause_btn = |state| {

            match self.state {
                visualizer::State::Paused => {
                    let label = iced::Text::new("Resume").size(16);
                    let button =
                        Button::new(state, label);
                    button.on_press(Message::Resume).padding(8)
                }
                visualizer::State::Running => {
                    let label = iced::Text::new("Pause").size(16);
                    let button =
                        Button::new(state, label);
                    button.on_press(Message::Pause).padding(8)
                }
            }

        };

        let filter_btn = |state, label, emit| {

            let label = iced::Text::new(label).size(16);
            let button =
                Button::new(state, label);
            button.on_press(emit).padding(8)

        };

        Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Alignment::Center)
            .push(
                Row::new()
                    .width(Length::Shrink)
                    .spacing(10)
                    .push(play_pause_btn(
                        play_pause,
                    ))
                    .push(filter_btn(
                        reset,
                        "Reset",
                        Message::Reset,
                    )),
            )
    }
}