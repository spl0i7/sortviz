use std::fmt;
use std::fmt::{Debug, Formatter};

mod bubble;
mod selection;
mod exchange;

pub use bubble::*;
pub use selection::*;
pub use exchange::*;

#[derive(Debug)]
pub struct StepResult<T> {
    pub items: Vec<T>,
    pub compared: (Option<usize>, Option<usize>),
}


pub trait StepSortingAlgorithm<T> where Self: Iterator<Item=StepResult<T>> {}

pub type IntegerSortingAlgorithm = Box<dyn StepSortingAlgorithm<i32, Item=StepResult<i32>>>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Algorithm {
    Bubble,
    Selection,
    Exchange
}

impl Algorithm {
    pub fn new(algo: Algorithm, elements: Vec<i32>) -> IntegerSortingAlgorithm {
        match algo {
            Algorithm::Bubble => {
                Box::new(BubbleSort::new(elements))
            }
            Algorithm::Selection => {
               Box::new(SelectionSort::new(elements))
            }
            Algorithm::Exchange => {
                Box::new(ExchangeSort::new(elements))
            }
        }
    }

    pub const ALL: [Algorithm; 3] = [
        Algorithm::Bubble,
        Algorithm::Selection,
        Algorithm::Exchange
    ];
}

impl Default for Algorithm {
    fn default() -> Self {
        Algorithm::Bubble
    }
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Algorithm::Bubble => "bubble",
                Algorithm::Selection => "selection",
                Algorithm::Exchange => "exchange"
            }
        )
    }
}





