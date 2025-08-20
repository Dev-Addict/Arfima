pub struct CircularBuffer<T> {
    size: usize,
    vec: Vec<T>,
    current: usize,
}

impl<T> CircularBuffer<T> {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            vec: Vec::with_capacity(size),
            current: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        self.current = (self.current + 1) % self.size;

        if self.vec.len() < self.size {
            self.vec.push(item);
        } else {
            self.vec[self.current] = item;
        }
    }

    pub fn get_from_current(&self, diff: isize) -> Option<&T> {
        let size = isize::try_from(self.size).unwrap_or(1);
        let diff: usize = (diff % size + size).try_into().unwrap_or(1);
        self.vec.get((self.current + diff) % self.size)
    }
}
