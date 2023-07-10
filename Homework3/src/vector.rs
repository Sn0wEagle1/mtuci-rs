pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Vector { data: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Vector {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn resize<F>(&mut self, new_capacity: usize, new_element: F)
    where
        F: Fn(usize) -> T,
    {
        let current_size = self.data.len();
        if new_capacity < current_size {
            self.data.truncate(new_capacity);
        } else if new_capacity > current_size {
            self.data.reserve(new_capacity - current_size);
            for i in current_size..new_capacity {
                self.data.push(new_element(i));
            }
        }
    }
}


fn main() {
    let mut vector: Vector<i32> = Vector::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    println!("{:?}", vector.pop());
}