use std::time::Duration;
use iced::*;
use iced::canvas::*;
use iced::window::Mode;
use rand::{Rng, thread_rng};
use crate::{BubbleSort, StepSortingAlgorithm};
use crate::algorithms::{Algorithm, IntegerSortingAlgorithm, SelectionSort, StepResult};

const REFRESH_TIME: Duration = Duration::from_millis(10);
const WIDTH: u16 = 1024;
const HEIGHT: u16 = 768;
const PADDING: u16 = 5;
const BAR_WIDTH: f32 = 15.0;

const NUM_BARS: usize = ((WIDTH as f32 - PADDING as f32) / (BAR_WIDTH as f32)) as usize;





enum State {
    Paused,
    Running,
}


pub struct SortingVisualizer {
    canvas_cache: Cache,
    state: State,
    algorithm: IntegerSortingAlgorithm,
    bars: Vec<i32>,
    compared_index: (usize, usize),
}


#[derive(Debug)]
pub enum Message {
    Step,
    Reset,
    Pause,
    Resume,
}

impl SortingVisualizer {
    fn generate_bars(sz: usize) -> Vec<i32> {
        let mut rng = thread_rng();
        let mut v = Vec::new();
        for i in 0..sz {
            v.push(rng.gen_range(10..700));
        }
        v
    }
}


impl Application for SortingVisualizer {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut bars = SortingVisualizer::generate_bars(NUM_BARS);

        (SortingVisualizer {
            canvas_cache: Cache::default(),
            state: State::Running,
            bars: bars.clone(),
            algorithm: Algorithm::new(Algorithm::Selection, bars),
            compared_index: (0, 0),
        }, Command::none()
        )
    }


    fn title(&self) -> String {
        String::from("Sorting visualizer")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Step => {
                match self.algorithm.next() {
                    None => {}
                    Some(e) => {
                        self.bars = e.items;
                        self.compared_index = (e.compared.0.unwrap_or(0), e.compared.1.unwrap_or(0))
                    }
                }
                self.canvas_cache.clear();
            }
            _ => {}
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        time::every(REFRESH_TIME).map(|_| {
            Message::Step
        })
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let canvas = Canvas::new(self)
            .width(Length::Units(WIDTH))
            .height(Length::Units(HEIGHT));



        Container::new(canvas)
            .width(Length::Units(WIDTH))
            .height(Length::Units(HEIGHT))
            .padding(Padding::new(PADDING))
            .into()
    }
}


impl<Message> canvas::Program<Message> for SortingVisualizer {
    fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry> {
        let canvas = self.canvas_cache.draw(bounds.size(), |frame| {
            let height = frame.height();

            let mut x = 0.0;

            for (idx, i) in self.bars.iter().enumerate() {
                x += BAR_WIDTH;

                let stroke: Stroke;

                if idx == self.compared_index.0 || idx == self.compared_index.1 {
                    stroke = Stroke {
                        width: BAR_WIDTH,
                        color: Color::new(1.0, 0.0, 0.0, 1.0),

                        ..Stroke::default()
                    };
                } else {
                    stroke = Stroke {
                        width: BAR_WIDTH,
                        color: Color::BLACK,
                        ..Stroke::default()
                    };
                }

                let bar =
                    Path::line(Point::new(x, height), Point::new(x, height - *i as f32));

                frame.stroke(&bar, stroke);
            }
        });

        vec![canvas]
    }
}
