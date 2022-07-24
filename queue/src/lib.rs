#[derive(Debug, PartialEq)]
pub struct Queue<T> {
    next: usize,
    last: usize,
    size: usize,
    ring: Box<[T]>,
}

impl<T> Queue<T> {
    pub fn new(size: usize) -> Self {
        assert_ne!(size, 0, "Queue requires a size of at least 1 element.");
        Self {
            next: 0,
            last: 0,
            size: 0,
            ring: Self::alloc(size),
        }
    }

    fn alloc(size: usize) -> Box<[T]> {
        let layout = std::alloc::Layout::array::<T>(size).unwrap();
        let start = unsafe { std::alloc::alloc(layout) as *mut T };
        let slice = core::ptr::slice_from_raw_parts_mut(start, size);
        unsafe { Box::from_raw(slice) }
    }

    fn grow(&mut self) {
        let Self {
            next,
            last,
            size,
            ring: old_ring,
            ..
        } = self;
        let mut new_ring = Self::alloc(*size * 2);
        if last < next {
            /*
              0 1 2 3
             |c|d|a|b|
                ^ ^
                | next
                last

             becomes...

              0 1 2 3 4 5 6 7
             |a|b|c|d| | | | |
              ^     ^
              next  last
            */
            let ranges = (*next..*size).chain(0..=*last);
            for (new_index, old_index) in ranges.enumerate() {
                std::mem::swap(&mut old_ring[old_index], &mut new_ring[new_index]);
            }
            self.next = 0;
            self.last = *size - 1;
        } else {
            for index in *next..=*last {
                std::mem::swap(&mut old_ring[index], &mut new_ring[index]);
            }
        };
        let _ = std::mem::replace(&mut self.ring, new_ring);
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn enqueue(&mut self, item: T) {
        if self.size == self.ring.len() {
            self.grow();
        }
        self.last = (self.last + 1) % self.ring.len();
        self.ring[self.last] = item;
        self.size += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let null = unsafe { std::mem::MaybeUninit::<T>::uninit().assume_init() };
        let item = std::mem::replace(&mut self.ring[self.next], null);
        self.next = (self.next + 1) % self.ring.len();
        self.size -= 1;
        Some(item)
    }

    pub fn peek(&self) -> Option<&T> {
        match self.size {
            0 => None,
            _ => Some(&self.ring[self.next]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    #[should_panic]
    fn test_new() {
        let _ = Queue::<i32>::new(0);
    }

    #[test]
    fn test_enqueue() {
        let mut queue = Queue::new(1);
        queue.enqueue(41);
        queue.enqueue(42);
        queue.enqueue(43);
        queue.enqueue(44);
        queue.enqueue(45);
        assert_eq!(queue.size(), 5);
    }

    #[test]
    fn test_dequeue() {
        let mut queue = Queue::new(1);
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
        assert_eq!(queue.dequeue(), Some(43));
        assert_eq!(queue.dequeue(), Some(44));
    }

    #[test]
    fn test_peek() {
        let mut queue = Queue::new(1);
        assert_eq!(queue.peek(), None);
        queue.enqueue(41);
        assert_eq!(queue.peek(), Some(&41));
        queue.enqueue(42);
        assert_eq!(queue.peek(), Some(&41));
        queue.dequeue();
        assert_eq!(queue.peek(), Some(&42));
    }
}
