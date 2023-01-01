use crate::algorithms::{StepResult, StepSortingAlgorithm};

pub struct ExchangeSort {
    pos_i: usize,
    pos_j: usize,
}

impl ExchangeSort {
    pub(crate) fn new() -> Self {
        ExchangeSort {
            pos_i: 0,
            pos_j: 1,
        }
    }
}

impl<T> StepSortingAlgorithm<T> for ExchangeSort where T: Ord  {
    fn sort_step(&mut self, items: &mut [T]) -> Option<StepResult> {
        if self.pos_i == items.len() {
            return None;
        }


        if self.pos_j == items.len() {
            let old_j = self.pos_j;

            self.pos_i += 1;
            self.pos_j = self.pos_i + 1;

            if old_j >= 2 {
                return Some(StepResult {
                    compared: (Some(self.pos_i - 1), Some(items.len() - 1))
                });
            }

            return Some(StepResult {
                compared: (None, None),
            });
        }

        if items[self.pos_i] > items[self.pos_j] {
            items.swap(self.pos_i, self.pos_j)
        }


        self.pos_j += 1;

        if self.pos_i >= 1 {
            return Some(StepResult {
                compared: (Some(self.pos_i - 1), Some(self.pos_j - 1)),
            });
        }


        return Some(StepResult {
            compared: (None, None),
        });
    }
}