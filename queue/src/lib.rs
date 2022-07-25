use dsa_rs_buffer::Buffer;

#[derive(Debug, PartialEq)]
pub struct Queue<T> {
    head: usize,
    tail: usize,
    size: usize,
    data: Buffer<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            head: 0,
            tail: 0,
            data: Buffer::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn enqueue(&mut self, elem: T) {
        if self.size == self.data.capacity() {
            self.data.grow();
            if self.size > 0 && self.tail == self.head {
                for index in 0..self.tail {
                    self.data.swap(index, index + self.size);
                }
                self.tail = self.tail + self.size;
            }
        }
        unsafe { self.data.write(self.tail, elem) };
        self.tail = (self.tail + 1) % self.data.capacity();
        self.size += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let elem = unsafe { self.data.read(self.head) };
        self.head = (self.head + 1) % self.data.capacity();
        self.size -= 1;
        Some(elem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_new() {
        let queue = Queue::<i32>::new();
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn test_enqueue() {
        let mut queue = Queue::new();
        queue.enqueue(41);
        queue.enqueue(42);
        queue.enqueue(43);
        queue.enqueue(44);
        queue.enqueue(45);
        assert_eq!(queue.size(), 5);
    }

    #[test]
    fn test_dequeue() {
        let mut queue = Queue::new();
        assert_eq!(queue.size(), 0);
        assert_eq!(queue.dequeue(), None);
        queue.enqueue(41);
        queue.enqueue(42);
        assert_eq!(queue.size(), 2);
        assert_eq!(queue.dequeue(), Some(41));
        assert_eq!(queue.size(), 1);
        queue.enqueue(43);
        assert_eq!(queue.size(), 2);
        assert_eq!(queue.dequeue(), Some(42));
        assert_eq!(queue.size(), 1);
        queue.enqueue(44);
        queue.enqueue(45);
        queue.enqueue(46);
        queue.enqueue(47);
        println!("{:?}", queue);
        assert_eq!(queue.dequeue(), Some(43));
        assert_eq!(queue.dequeue(), Some(44));
        assert_eq!(queue.dequeue(), Some(45));
        assert_eq!(queue.dequeue(), Some(46));
        assert_eq!(queue.dequeue(), Some(47));
        assert_eq!(queue.dequeue(), None);
    }
}
