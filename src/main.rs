#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

use iced::{Application, Settings};
use crate::algorithms::{BubbleSort, StepSortingAlgorithm};
use crate::visualizer::SortingVisualizer;

mod visualizer;
mod algorithms;

fn main() {
    let _ = SortingVisualizer::run(Settings {
        antialiasing: true,
        ..Settings::default()
    });
    
}
