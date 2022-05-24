use std::time::Duration;
use iced::*;
use iced::canvas::*;
use rand::{Rng, thread_rng};
use crate::algorithms::*;
use crate::visualizer::*;

const WIDTH: u16 = 1024;
const HEIGHT: u16 = 768;
const PADDING: u16 = 5;
const BAR_WIDTH: f32 = 15.0;

const NUM_BARS: usize = ((WIDTH as f32 - PADDING as f32) / (BAR_WIDTH as f32)) as usize;

const REFRESH_TIME: Duration = Duration::from_millis(10);


pub struct Painting {
    canvas_cache: Cache,
    algorithm: IntegerSortingAlgorithm,
    bars: Vec<i32>,
    compared_index: (usize, usize),
}

impl Painting {
    fn generate_bars(sz: usize) -> Vec<i32> {
        let mut rng = thread_rng();
        let mut v = Vec::new();
        for i in 0..sz {
            v.push(rng.gen_range(10..700));
        }
        v
    }

    pub fn new() -> Self {
        let mut bars = Painting::generate_bars(NUM_BARS);

        Painting {
            canvas_cache: Cache::default(),
            bars: bars.clone(),
            algorithm: Algorithm::new(Algorithm::Selection, bars),
            compared_index: (0, 0),
        }
    }


    pub fn request_redraw(&mut self) {
        self.canvas_cache.clear()
    }

    pub fn sort_step(&mut self) {
        match self.algorithm.next() {
            None => {}
            Some(e) => {
                self.bars = e.items;
                self.compared_index = (e.compared.0.unwrap_or(0), e.compared.1.unwrap_or(0))
            }
        }
        self.canvas_cache.clear();
    }
    pub fn view(&mut self) -> Container<Message> {
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


impl<Message> canvas::Program<Message> for Painting {
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
