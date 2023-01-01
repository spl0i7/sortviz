use crate::algorithms::{StepResult, StepSortingAlgorithm};

pub struct GnomeSort {
    pos_i: usize,
}

impl GnomeSort {
    pub(crate) fn new() -> Self {
        GnomeSort {
            pos_i: 0,
        }
    }
}

impl<T> StepSortingAlgorithm<T> for GnomeSort where T: Ord {
    fn sort_step(&mut self, items: &mut [T]) -> Option<StepResult> {
        if self.pos_i == items.len() {
            return None;
        }

        if self.pos_i == 0 || items[self.pos_i] >= items[self.pos_i - 1] {
            self.pos_i += 1;
            return Some(StepResult {
                compared: (Some(self.pos_i), Some(self.pos_i - 1))
            });
        } else {
            items.swap(self.pos_i, self.pos_i - 1);
            self.pos_i -= 1;
            return Some(StepResult {
                compared: (Some(self.pos_i + 1), Some(self.pos_i))
            });
        }
    }
}