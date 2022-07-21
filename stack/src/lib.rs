#[derive(Debug, PartialEq)]
pub struct Stack<T> {
    size: usize,
    items: Box<[T]>,
}

impl<T: Default> Stack<T> {
    pub fn new(size: usize) -> Self {
        if size < 1 {
            panic!("Stack requires a size of at least 1 element.")
        }
        Stack {
            size: 0,
            items: Self::alloc(size),
        }
    }

    fn alloc(size: usize) -> Box<[T]> {
        let mut items = Vec::<T>::with_capacity(size);
        items.resize_with(size, T::default);
        items.into_boxed_slice()
    }

    fn grow(&mut self) {
        let mut items = Self::alloc(self.size * 2);
        for index in 0..self.size {
            std::mem::swap(&mut self.items[index], &mut items[index]);
        }
        self.items = items;
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, item: T) {
        if self.size == self.items.len() {
            self.grow();
        }
        self.items[self.size] = item;
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        let item = std::mem::replace(&mut self.items[self.size], T::default());
        Some(item)
    }

    pub fn peek(&self) -> Option<&T> {
        match self.size {
            0 => None,
            _ => Some(&self.items[self.size - 1]),
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
