use std::fmt::Debug;

#[derive(Debug)]
pub struct BinaryHeap<T> {
    data: Vec<T>,
}

impl<T: Ord> BinaryHeap<T> {
    pub fn new() -> Self {
        BinaryHeap { data: vec![] }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        BinaryHeap {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Removes the greatest item from the binary heap and returns it, or `None` if it is empty.
    ///
    //    pub fn pop(&mut self) -> Option<T> {}

    pub fn push(&mut self, item: T) {
        self.data.push(item);

        let mut i = 0i32;
        loop {
            let parent = (i - 1) / 2;

            if i != 0 && self.data[parent as usize] > self.data[i as usize] {
                self.data.swap(i as usize, parent as usize);
                i = parent;
            } else {
                break;
            }
        }
    }

    fn heapify(&mut self, index: usize) {
        let lhs = 2 * index + 1;
        let rhs = 2 * index + 2;
        let mut smallest = index;
        if self.data[lhs] < self.data[index] {
            //  lhs < self.size &&
            smallest = lhs
        }
        if self.data[rhs] < self.data[smallest] {
            smallest = rhs
        }
        if smallest != index {
            self.data.swap(index, smallest);
            self.heapify(smallest);
        }
    }
}
