use std::fmt::Debug;
use crate::algorithms::{StepResult, StepSortingAlgorithm};

pub struct BubbleSort<T> where T: Ord + Clone + Debug {
    elements: Vec<T>,
    pos_i: usize,
    pos_j: usize,
}

impl<T> BubbleSort<T>  where T: Ord + Clone + Debug  {
    pub(crate) fn new(elements: Vec<T>) -> Self {
        BubbleSort {
            elements,
            pos_i: 0,
            pos_j: 0
        }
    }
}

impl<T> StepSortingAlgorithm<T> for BubbleSort<T> where T: Ord + Debug + Clone{}

impl<T> Iterator for BubbleSort<T>
    where T: Ord + Clone + Debug {
    type Item = StepResult<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos_i == self.elements.len() {
            return None;
        }
        

        if self.pos_j == self.elements.len() - self.pos_i - 1 {
            let old_j = self.pos_j;

            self.pos_i += 1;
            self.pos_j = 0;

            if old_j >= 2 {
                return Some(StepResult {
                    items: self.elements.clone(),
                    compared: (Some(old_j -1),Some(old_j -2)),
                });
            }

            return Some(StepResult {
                items: self.elements.clone(),
                compared: (None, None),
            });
        }

        if self.elements[self.pos_j].gt(&self.elements[self.pos_j + 1]) {
            self.elements.swap(self.pos_j, self.pos_j + 1)
        }


        self.pos_j += 1;

        if self.pos_j >= 2 {
            return Some(StepResult {
                items: self.elements.clone(),
                compared: (Some(self.pos_j -1),Some(self.pos_j -2)),
            });
        }


        return Some(StepResult {
            items: self.elements.clone(),
            compared: (None, None),
        });
    }
}

#[test]
fn test_bubble_sort() {
    let v = vec![9, 8, 6, 5, 3, 1, 0];

    let steps = BubbleSort::new(v);

    let result: Vec<Vec<i32>> = steps.map(|x| x.items).collect();

    assert_eq!(*result.last().unwrap(), vec![0, 1, 3, 5, 6, 8, 9]);

}