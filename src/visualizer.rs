use std::time::Duration;
use iced::*;
use crate::algorithms::Algorithm;
use crate::controls::*;
use crate::painting::*;

const REFRESH_TIME: Duration = Duration::from_millis(10);

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum State {
    Paused,
    Running,
}

pub struct SortingVisualizer {
    painting: Painting,
    state: State,
    controls: Controls,
}


#[derive(Debug, Clone)]
pub enum Message {
    Step,
    Reset,
    Pause,
    Resume,
    Algorithm(Algorithm)
}

pub enum ControlMessage {
    StateChanged(State),
    AlgorithmChanged(Algorithm),
}


impl Application for SortingVisualizer {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (SortingVisualizer {
            painting: Painting::new(Algorithm::default()),
            state: State::Running,
            controls: Controls::new(State::Running),
        }, Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Sorting visualizer")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Step => {
                if self.state == State::Paused {
                    return Command::none()
                }
                self.painting.sort_step();
                self.painting.request_redraw();
            }
            Message::Pause => {
                self.state = State::Paused;
                self.controls.update(ControlMessage::StateChanged(self.state));
            },
            Message::Resume =>  {
                self.state = State::Running;
                self.controls.update(ControlMessage::StateChanged(self.state));
            },
            Message::Reset => {
                self.state = State::Paused;
                self.controls.update(ControlMessage::StateChanged(self.state));
                self.painting = Painting::new(Algorithm::default());
                self.painting.request_redraw();
            },

            Message::Algorithm(a) => {
                self.state = State::Paused;
                self.controls.update(ControlMessage::StateChanged(self.state));
                self.controls.update(ControlMessage::AlgorithmChanged(a));
                self.painting = Painting::new(a);
                self.painting.request_redraw();
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        time::every(REFRESH_TIME).map(|_| {
            Message::Step
        })
    }

    fn view(&mut self) -> Element<'_, Self::Message> {

        Row::new()
            .push(self.painting.view())
            .push(self.controls.view())
            .into()
    }
}
