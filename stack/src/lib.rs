#[derive(Debug, PartialEq)]
pub struct Stack<T> {
    size: usize,
    items: Box<[T]>,
}

impl<T: Copy + Default> Stack<T> {
    fn new<const N: usize>() -> Self {
        if N < 1 {
            panic!("Stack requires a size of at least 1 element.")
        }
        Stack {
            size: 0,
            items: Box::new([T::default(); N]),
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.size {
            0 => None,
            _ => Some(&self.items[self.size - 1]),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, item: T) {
        if self.size == self.items.len() {
            let new_len = self.items.len() * 2;
            let layout = std::alloc::Layout::array::<T>(new_len).unwrap();
            unsafe {
                let start = std::alloc::alloc(layout) as *mut T;
                let slice = std::slice::from_raw_parts_mut(start, new_len);
                for i in 0..self.items.len() {
                    slice[i] = self.items[i];
                }
                for i in self.items.len()..slice.len() {
                    slice[i] = T::default();
                }
                let _ = std::mem::replace(&mut self.items, Box::from_raw(slice));
            }
        }
        self.items[self.size] = item;
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.size {
            0 => None,
            _ => {
                self.size -= 1;
                let item = self.items[self.size];
                self.items[self.size] = T::default();
                Some(item)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_new() {
        let _: Stack<i32> = Stack::new::<0>();
    }

    #[test]
    fn test_push() {
        let mut stack: Stack<i32> = Stack::new::<1>();
        stack.push(41);
        stack.push(42);
        stack.push(43);
        stack.push(44);
        stack.push(45);
        assert_eq!(stack.size(), 5);
    }

    #[test]
    fn test_pop() {
        let mut stack: Stack<i32> = Stack::new::<1>();
        assert_eq!(stack.pop(), None);
        stack.push(42);
        assert_eq!(stack.pop(), Some(42));
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_peek() {
        let mut stack: Stack<i32> = Stack::new::<1>();
        assert_eq!(stack.peek(), None);
        stack.push(41);
        stack.push(42);
        assert_eq!(stack.peek(), Some(&42));
    }
}
