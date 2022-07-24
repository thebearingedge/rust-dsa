#[derive(Debug, PartialEq)]
pub struct Stack<T> {
    size: usize,
    buf: Box<[T]>,
}

impl<T> Stack<T> {
    pub fn new(size: usize) -> Self {
        assert_ne!(size, 0, "Stack requires a size of at least 1 element.");
        Self {
            size: 0,
            buf: Self::alloc(size),
        }
    }

    fn alloc(size: usize) -> Box<[T]> {
        let layout = std::alloc::Layout::array::<T>(size).unwrap();
        let start = unsafe { std::alloc::alloc(layout) as *mut T };
        let slice = std::ptr::slice_from_raw_parts_mut(start, size);
        unsafe { Box::from_raw(slice) }
    }

    fn grow(&mut self) {
        let mut items = Self::alloc(self.size * 2);
        for index in 0..self.size {
            std::mem::swap(&mut self.buf[index], &mut items[index]);
        }
        let _ = std::mem::replace(&mut self.buf, items);
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, item: T) {
        if self.size == self.buf.len() {
            self.grow();
        }
        self.buf[self.size] = item;
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        let null = unsafe { std::mem::MaybeUninit::<T>::uninit().assume_init() };
        let item = std::mem::replace(&mut self.buf[self.size], null);
        Some(item)
    }

    pub fn peek(&self) -> Option<&T> {
        match self.size {
            0 => None,
            _ => Some(&self.buf[self.size - 1]),
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
        let _ = Stack::<i32>::new(0);
    }

    #[test]
    fn test_push() {
        let mut stack = Stack::new(1);
        stack.push(41);
        stack.push(42);
        stack.push(43);
        stack.push(44);
        stack.push(45);
        assert_eq!(stack.size(), 5);
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack::new(1);
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.pop(), None);
        stack.push(42);
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.pop(), Some(42));
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_peek() {
        let mut stack = Stack::new(1);
        assert_eq!(stack.peek(), None);
        stack.push(41);
        assert_eq!(stack.peek(), Some(&41));
        stack.push(42);
        assert_eq!(stack.peek(), Some(&42));
        stack.pop();
        assert_eq!(stack.peek(), Some(&41));
    }
}
