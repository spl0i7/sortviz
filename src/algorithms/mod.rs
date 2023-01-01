use std::fmt;
use std::fmt::{Debug, Formatter};

mod bubble;
mod selection;
mod exchange;
mod gnome;

pub use bubble::*;
pub use selection::*;
pub use exchange::*;
use crate::algorithms::gnome::GnomeSort;

#[derive(Debug)]
pub struct StepResult {
    pub compared: (Option<usize>, Option<usize>),
}


pub trait StepSortingAlgorithm<T> {
    fn sort_step(&mut self, items: &mut[T]) -> Option<StepResult>;
}

pub type IntegerSortingAlgorithm = Box<dyn StepSortingAlgorithm<i32>>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Algorithm {
    Bubble,
    Selection,
    Exchange,
    Gnome
}

impl Algorithm {
    pub fn new(algo: Algorithm) -> IntegerSortingAlgorithm {
        match algo {
            Algorithm::Bubble => {
                Box::new(BubbleSort::new())
            }
            Algorithm::Selection => {
               Box::new(SelectionSort::new())
            }
            Algorithm::Exchange => {
                Box::new(ExchangeSort::new())
            }
            Algorithm::Gnome => {
                Box::new(GnomeSort::new())
            }
        }
    }

    pub const ALL: [Algorithm; 4] = [
        Algorithm::Bubble,
        Algorithm::Selection,
        Algorithm::Exchange,
        Algorithm::Gnome,
    ];
}

impl Default for Algorithm {
    fn default() -> Self {
        Algorithm::Gnome
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
                Algorithm::Exchange => "exchange",
                Algorithm::Gnome => "gnome"
            }
        )
    }
}





