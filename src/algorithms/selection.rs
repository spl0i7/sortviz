use crate::algorithms::{StepResult, StepSortingAlgorithm};

#[derive(Default)]
pub struct SelectionSort {
    pos_i: usize,
    pos_j: usize,
    min_index: usize,
}


impl SelectionSort {
    pub(crate) fn new() -> Self {
        SelectionSort {
            pos_i: 0,
            pos_j: 1,
            min_index: 0
        }
    }
}

impl<T> StepSortingAlgorithm<T> for SelectionSort where T: Ord {
    fn sort_step(&mut self, items: &mut [T]) -> Option<StepResult> {
        if self.pos_i == items.len() {
            return None;
        }

        if self.pos_j == items.len() {
            items.swap(self.pos_i, self.min_index);


            self.pos_i += 1;
            self.pos_j = self.pos_i + 1;
            self.min_index = self.pos_i;

            return Some(StepResult {
                compared: (Some(items.len() - 1), Some(self.min_index)),
            });
        }

        if items[self.pos_j].lt(&items[self.min_index]) {
            self.min_index = self.pos_j;
        }

        self.pos_j += 1;

        return Some(StepResult {
            compared: (Some(self.pos_j), Some(self.min_index)),
        });
    }
}