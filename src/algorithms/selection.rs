use std::fmt::Debug;
use crate::algorithms::{StepResult, StepSortingAlgorithm};

#[derive(Default)]
pub struct SelectionSort<T> where T: Ord + Clone + Debug {
    elements: Vec<T>,
    pos_i: usize,
    pos_j: usize,
    min_index: usize,
}


impl<T> SelectionSort<T>  where T: Ord + Clone + Debug {
    pub(crate) fn new(elements: Vec<T>) -> Self {
        SelectionSort {
            elements,
            pos_i: 0,
            pos_j: 1,
            min_index: 0
        }
    }
}

impl<T> StepSortingAlgorithm<T> for SelectionSort<T> where T: Ord + Debug + Clone{}

impl<T> Iterator for SelectionSort<T>
    where T: Ord + Clone + Debug {
    type Item = StepResult<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos_i == self.elements.len() {
            return None;
        }


        if self.pos_j == self.elements.len() {
            self.elements.swap(self.pos_i, self.min_index);


            self.pos_i += 1;
            self.pos_j = self.pos_i + 1;
            self.min_index = self.pos_i;

            return Some(StepResult {
                items: self.elements.clone(),
                compared: (Some(self.elements.len() - 1), Some(self.min_index)),
            });
        }


        if self.elements[self.pos_j].lt(&self.elements[self.min_index]) {
            self.min_index = self.pos_j;
        }

        self.pos_j += 1;

        return Some(StepResult {
            items: self.elements.clone(),
            compared: (Some(self.pos_j), Some(self.min_index)),
        });
    }
}

#[test]
fn test_selection_sort() {
    let v = vec![9, 8, 6, 5, 3, 1, 0];

    let steps = SelectionSort::new(v);

    let result: Vec<Vec<i32>> = steps.map(|x| x.items).collect();

    assert_eq!(*result.last().unwrap(), vec![0, 1, 3, 5, 6, 8, 9]);
}