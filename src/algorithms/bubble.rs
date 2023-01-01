use crate::algorithms::{StepResult, StepSortingAlgorithm};

pub struct BubbleSort {
    pos_i: usize,
    pos_j: usize,
}

impl BubbleSort  {
    pub(crate) fn new() -> Self {
        BubbleSort {
            pos_i: 0,
            pos_j: 0,
        }
    }
}

impl<T> StepSortingAlgorithm<T> for BubbleSort where T: Ord {
    fn sort_step(&mut self, items: &mut [T]) -> Option<StepResult> {
        if self.pos_i == items.len() {
            return None;
        }


        if self.pos_j == items.len() - self.pos_i - 1 {
            let old_j = self.pos_j;

            self.pos_i += 1;
            self.pos_j = 0;

            if old_j >= 2 {
                return Some(StepResult {
                    compared: (Some(old_j - 1), Some(old_j - 2)),
                });
            }

            return Some(StepResult {
                compared: (None, None),
            });
        }

        if items[self.pos_j] > items[self.pos_j + 1] {
            items.swap(self.pos_j, self.pos_j + 1)
        }


        self.pos_j += 1;

        if self.pos_j >= 2 {
            return Some(StepResult {
                compared: (Some(self.pos_j - 1), Some(self.pos_j - 2)),
            });
        }

        return Some(StepResult {
            compared: (None, None),
        });
    }
}