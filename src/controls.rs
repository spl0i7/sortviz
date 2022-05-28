use iced::*;
use iced::Length::FillPortion;
use crate::algorithms::Algorithm;
use crate::visualizer;
use crate::visualizer::{Message};

#[derive(Debug, Clone)]
pub enum ControlState {
    Paused,
    Running,
}

pub enum ControlMessage {
    StateChanged(ControlState),
    AlgorithmChanged(Algorithm),
}

#[derive(Debug, Clone)]
pub struct Controls {
    state: ControlState,
    selected_algo: Algorithm,
    pick_list: pick_list::State<Algorithm>,
    play_pause: button::State,
    reset: button::State,
}

impl Controls {
    pub fn new(state: ControlState) -> Self {
        Controls {
            state,
            pick_list: pick_list::State::default(),
            selected_algo: Algorithm::default(),
            play_pause: button::State::default(),
            reset: button::State::default(),
        }
    }

    pub fn update(&mut self, message: ControlMessage) {
        match message {
            ControlMessage::StateChanged(s) => {
                self.state = s;
            }
            ControlMessage::AlgorithmChanged(a) => {
                self.selected_algo = a;
            }
        }
    }

    pub fn view(&mut self) -> Column<Message> {
        let Controls {
            state: _,
            pick_list: _,
            selected_algo: _,
            play_pause,
            reset,
        } = self;

        let play_pause_btn = |state| {
            match self.state {
                ControlState::Paused => {
                    let label = Text::new("Play").size(16);
                    let button =
                        Button::new(state, label);
                    button.on_press(Message::Resume).padding(8)
                }
                ControlState::Running => {
                    let label = Text::new("Pause").size(16);
                    let button =
                        Button::new(state, label);
                    button.on_press(Message::Pause).padding(8)
                }
            }
        };

        let reset_btn = |state, label, emit| {
            let label = Text::new(label).size(16);
            let button =
                Button::new(state, label);
            button.on_press(emit).padding(8)
        };

        let pick_list = PickList::new(
            &mut self.pick_list,
            &Algorithm::ALL[..],
            Option::from(self.selected_algo),
            visualizer::Message::Algorithm,
        );

        Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Alignment::Center)
            .push(pick_list)
            .width(FillPortion(3))
            .push(
                Row::new()
                    .width(Length::Shrink)
                    .spacing(10)
                    .push(play_pause_btn(
                        play_pause,
                    ))
                    .push(reset_btn(
                        reset,
                        "Reset",
                        Message::Reset,
                    )),
            )
    }
}