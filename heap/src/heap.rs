#[derive(Debug)]
pub struct Heapi32 {
    data: Vec<i32>,
}

impl Heapi32 {
    pub fn push(&mut self, val: i32) -> () {
        self.data.push(val);
        let mut i = &self.data.len() - 1;
        while i > 0 {
            let parent_index = (i - 1) / 2;
            // min heap
            if self.data[i] >= self.data[parent_index] {
                break;
            } else {
                //swap
                let tmp = self.data[i];
                self.data[i] = self.data[parent_index];
                self.data[parent_index] = tmp;
                i = parent_index;
            }
        }
    }

    pub fn peek(&self) -> Option<&i32> {
        self.data.first()
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            return None;
        }

        // Swap root with last element and remove it
        let n = self.data.len();
        self.data.swap(0, n - 1);
        let root = self.data.pop().unwrap();

        let mut i = 0;

        loop {
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            let n = self.data.len();

            // No children: heap property is restored
            if left >= n {
                break;
            }

            // Find the smaller child
            let smaller_child = if right < n && self.data[right] < self.data[left] {
                right
            } else {
                left
            };

            // Parent is already smaller than both children
            if self.data[i] <= self.data[smaller_child] {
                break;
            }

            // Swap parent with smaller child
            self.data.swap(i, smaller_child);
            i = smaller_child;
        }

        Some(root)
    }

    fn heapify(arr: Vec<i32>) -> Heapi32 {
        let mut heap = Heapi32 { data: vec![] };
        for val in arr {
            heap.push(val);
        }
        heap
    }
}

#[test]
fn test_push() {
    let mut heap = Heapi32 { data: vec![] };
    heap.push(5);
    heap.push(4);
    heap.push(1);
    heap.push(7);
    heap.push(10);
    assert_eq!(heap.peek(), Some(&1))
}

#[test]
fn test_peek() {
    let mut heap = Heapi32 { data: vec![] };
    heap.push(123);
    assert_eq!(heap.peek(), Some(&123))
}

#[test]
fn test_pop() {
    let mut heap = Heapi32 { data: vec![] };
    heap.push(5);
    heap.push(4);
    heap.push(1);
    heap.push(7);
    heap.push(10);
    assert_eq!(heap.pop(), Some(1));
    assert_eq!(heap.pop(), Some(4))
}

#[test]
fn test_heapify() {
    let v = vec![1, 2, 3, 4, 5, -1];
    let heap = Heapi32::heapify(v);
    assert_eq!(heap.peek(), Some(&-1))
}
