use std::fmt::Debug;
use iced::Application;
use crate::algorithms::{StepResult, StepSortingAlgorithm};

pub struct GnomeSort<T> where T: Ord + Clone + Debug {
    elements: Vec<T>,
    pos_i: usize,
    pos_j: usize,
}

impl<T> GnomeSort<T>  where T: Ord + Clone + Debug  {
    pub(crate) fn new(elements: Vec<T>) -> Self {
        GnomeSort {
            elements,
            pos_i: 0,
            pos_j: 1
        }
    }
}

impl<T> StepSortingAlgorithm<T> for GnomeSort<T> where T: Ord + Debug + Clone{}

impl<T> Iterator for GnomeSort<T>
    where T: Ord + Clone + Debug {
    type Item = StepResult<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos_i == self.elements.len() {
            return None;
        }


        if self.pos_j == self.elements.len()  {
            let old_j = self.pos_j;

            self.pos_i += 1;
            self.pos_j = self.pos_i + 1;

            if old_j >= 2 {
                return Some(StepResult {
                    items: self.elements.clone(),
                    compared: (Some(self.pos_i -1), Some(self.elements.len() - 1))
                });
            }

            return Some(StepResult {
                items: self.elements.clone(),
                compared: (None, None),
            });
        }

        if self.elements[self.pos_i].gt(&self.elements[self.pos_j]) {
            self.elements.swap(self.pos_i, self.pos_j)
        }


        self.pos_j += 1;

        if self.pos_i >= 1 {
            return Some(StepResult {
                items: self.elements.clone(),
                compared: (Some(self.pos_i -1), Some(self.pos_j - 1)),
            });
        }


        return Some(StepResult {
            items: self.elements.clone(),
            compared: (None, None),
        });
    }
}

#[test]
fn test_gnome_sort() {
    let v = vec![9, 8, 6, 5, 3, 1, 0];

    let steps = GnomeSort::new(v);

    let result: Vec<Vec<i32>> = steps.map(|x| x.items).collect();

    assert_eq!(*result.last().unwrap(), vec![0, 1, 3, 5, 6, 8, 9]);

}