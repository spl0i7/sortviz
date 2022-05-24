
use iced::{Application, Settings};
use crate::algorithms::{BubbleSort, StepSortingAlgorithm};
use crate::visualizer::SortingVisualizer;

mod visualizer;
mod algorithms;
mod controls;
mod painting;


fn main() {
    let _ = SortingVisualizer::run(Settings {
        antialiasing: true,
        ..Settings::default()
    });
    
}
