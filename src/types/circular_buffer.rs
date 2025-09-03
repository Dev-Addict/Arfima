pub struct CircularBuffer<T> {
    size: usize,
    vec: Vec<T>,
    current: usize,
}

impl<T: Default> CircularBuffer<T> {
    pub fn new(size: usize) -> Self {
        Self {
            size: size.max(1),
            vec: Vec::with_capacity(size),
            current: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.vec.len() < self.size {
            self.vec.push(item);
        } else {
            self.vec[self.current] = item;
        }

        self.current = (self.current + 1) % self.size;
    }

    pub fn get_from_current(&self, diff: isize) -> Option<&T> {
        if self.vec.is_empty() {
            return None;
        }

        let size = isize::try_from(self.size.min(self.vec.len())).unwrap_or(1);
        let diff: usize = (diff).rem_euclid(size).try_into().unwrap_or(1);
        self.vec.get((self.current + diff) % size.unsigned_abs())
    }

    pub fn set_size(&mut self, mut size: usize) {
        size = size.max(1);

        match size {
            size if size > self.size => {
                let mut new_vec = Vec::with_capacity(size);

                if self.current > 1 {
                    while self.current - 1 < self.vec.len() {
                        new_vec.push(self.vec.remove(self.current - 1));
                    }
                }

                new_vec.append(&mut self.vec);

                self.vec = new_vec;
                self.current = self.size;
            }
            size if size < self.size => match self.vec.len() {
                len if len > size => {
                    let mut new_vec = Vec::with_capacity(size);
                    self.current = self.current.min(size);
                    let from_end = size - self.current;

                    new_vec.extend(self.vec.drain((len - from_end)..));
                    new_vec.extend(self.vec.drain(0..self.current));

                    self.vec = new_vec;
                    self.current = 0;
                }
                _ => {}
            },
            _ => {}
        }

        self.size = size;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_normal_size() {
        let buffer: CircularBuffer<i32> = CircularBuffer::new(5);
        assert_eq!(buffer.size, 5);
        assert_eq!(buffer.vec.len(), 0);
        assert_eq!(buffer.current, 0);
    }

    #[test]
    fn test_new_zero_size_becomes_one() {
        let buffer: CircularBuffer<i32> = CircularBuffer::new(0);
        assert_eq!(buffer.size, 1);
    }

    #[test]
    fn test_new_capacity_allocated() {
        let buffer: CircularBuffer<String> = CircularBuffer::new(10);
        assert_eq!(buffer.vec.capacity(), 10);
    }

    #[test]
    fn test_push_to_empty_buffer() {
        let mut buffer = CircularBuffer::new(3);
        buffer.push(1);

        assert_eq!(buffer.vec.len(), 1);
        assert_eq!(buffer.vec[0], 1);
        assert_eq!(buffer.current, 1);
    }

    #[test]
    fn test_push_fill_buffer() {
        let mut buffer = CircularBuffer::new(3);

        buffer.push(1);
        buffer.push(2);
        buffer.push(3);

        assert_eq!(buffer.vec, vec![1, 2, 3]);
        assert_eq!(buffer.current, 0);
    }

    #[test]
    fn test_push_wrap_around() {
        let mut buffer = CircularBuffer::new(3);

        buffer.push(1);
        buffer.push(2);
        buffer.push(3);

        buffer.push(4);
        assert_eq!(buffer.vec, vec![4, 2, 3]);
        assert_eq!(buffer.current, 1);

        buffer.push(5);
        assert_eq!(buffer.vec, vec![4, 5, 3]);
        assert_eq!(buffer.current, 2);

        buffer.push(6);
        assert_eq!(buffer.vec, vec![4, 5, 6]);
        assert_eq!(buffer.current, 0);
    }

    #[test]
    fn test_push_single_element_buffer() {
        let mut buffer = CircularBuffer::new(1);

        buffer.push(1);
        assert_eq!(buffer.vec, vec![1]);
        assert_eq!(buffer.current, 0);

        buffer.push(2);
        assert_eq!(buffer.vec, vec![2]);
        assert_eq!(buffer.current, 0);
    }

    #[test]
    fn test_get_from_current_empty_buffer() {
        let buffer: CircularBuffer<i32> = CircularBuffer::new(3);
        assert_eq!(buffer.get_from_current(0), None);
        assert_eq!(buffer.get_from_current(1), None);
        assert_eq!(buffer.get_from_current(-1), None);
    }

    #[test]
    fn test_get_from_current_positive_offset() {
        let mut buffer = CircularBuffer::new(4);
        buffer.push(10);
        buffer.push(20);
        buffer.push(30);

        assert_eq!(buffer.get_from_current(0), Some(&10));
        assert_eq!(buffer.get_from_current(1), Some(&20));
        assert_eq!(buffer.get_from_current(2), Some(&30));
        assert_eq!(buffer.get_from_current(3), Some(&10));
    }

    #[test]
    fn test_get_from_current_negative_offset() {
        let mut buffer = CircularBuffer::new(3);
        buffer.push(10);
        buffer.push(20);
        buffer.push(30);

        assert_eq!(buffer.get_from_current(-1), Some(&30));
        assert_eq!(buffer.get_from_current(-2), Some(&20));
        assert_eq!(buffer.get_from_current(-3), Some(&10));
    }

    #[test]
    fn test_get_from_current_wrapping_offset() {
        let mut buffer = CircularBuffer::new(3);
        buffer.push(10);
        buffer.push(20);
        buffer.push(30);

        assert_eq!(buffer.get_from_current(3), Some(&10));
        assert_eq!(buffer.get_from_current(4), Some(&20));
        assert_eq!(buffer.get_from_current(-4), Some(&30));
    }

    #[test]
    fn test_set_size_grow_buffer() {
        let mut buffer = CircularBuffer::new(2);
        buffer.push(1);
        buffer.push(2);

        buffer.set_size(4);

        assert_eq!(buffer.size, 4);
        assert_eq!(buffer.current, 2);
        assert_eq!(buffer.vec, vec![1, 2]);
    }

    #[test]
    fn test_set_size_grow_with_wrapping() {
        let mut buffer = CircularBuffer::new(3);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.push(4);

        buffer.set_size(5);

        assert_eq!(buffer.size, 5);
        assert_eq!(buffer.vec, vec![4, 2, 3]);
    }

    #[test]
    fn test_set_size_shrink_buffer_fewer_items() {
        let mut buffer = CircularBuffer::new(5);
        buffer.push(1);
        buffer.push(2);

        buffer.set_size(3);

        assert_eq!(buffer.size, 3);
        assert_eq!(buffer.vec, vec![1, 2]);
    }

    #[test]
    fn test_set_size_shrink_buffer_more_items() {
        let mut buffer = CircularBuffer::new(5);
        for i in 1..=5 {
            buffer.push(i);
        }
        buffer.push(6);

        buffer.set_size(3);

        assert_eq!(buffer.size, 3);
        assert_eq!(buffer.current, 0);
        assert_eq!(buffer.vec, vec![4, 5, 6]);
    }

    #[test]
    fn test_set_size_same_size() {
        let mut buffer = CircularBuffer::new(3);
        buffer.push(1);
        buffer.push(2);
        let original_vec = buffer.vec.clone();
        let original_current = buffer.current;

        buffer.set_size(3);

        assert_eq!(buffer.size, 3);
        assert_eq!(buffer.vec, original_vec);
        assert_eq!(buffer.current, original_current);
    }

    #[test]
    fn test_set_size_zero_becomes_one() {
        let mut buffer = CircularBuffer::new(3);
        buffer.push(1);

        buffer.set_size(0);

        assert_eq!(buffer.size, 1);
    }

    #[test]
    fn test_integration_complex_scenario() {
        let mut buffer = CircularBuffer::new(3);

        for i in 1..=5 {
            buffer.push(i);
        }

        assert_eq!(buffer.get_from_current(0), Some(&3));
        assert_eq!(buffer.get_from_current(1), Some(&4));
        assert_eq!(buffer.get_from_current(-1), Some(&5));

        buffer.set_size(5);
        assert_eq!(buffer.vec, vec![5, 3, 4]);

        buffer.push(6);
        buffer.push(7);

        buffer.set_size(3);
        assert_eq!(buffer.size, 3);
        assert_eq!(buffer.vec.len(), 3);
    }

    #[test]
    fn test_string_type() {
        let mut buffer = CircularBuffer::new(2);

        buffer.push("hello".to_string());
        buffer.push("world".to_string());
        buffer.push("rust".to_string());

        assert_eq!(buffer.get_from_current(0), Some(&"world".to_string()));
        assert_eq!(buffer.get_from_current(-1), Some(&"rust".to_string()));
    }
}
