use dsa_rs_buffer::Buffer;

#[derive(Debug, PartialEq)]
pub struct Stack<T> {
    size: usize,
    data: Buffer<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            data: Buffer::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, elem: T) {
        if self.size == self.data.capacity() {
            self.data.grow();
        }
        unsafe { self.data.write(self.size, elem) };
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        Some(unsafe { self.data.read(self.size) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_new() {
        let stack = Stack::<()>::new();
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_push() {
        let mut stack = Stack::new();
        stack.push(41);
        stack.push(42);
        stack.push(43);
        stack.push(44);
        stack.push(45);
        assert_eq!(stack.size(), 5);
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack::new();
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.pop(), None);
        stack.push(42);
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.pop(), Some(42));
        assert_eq!(stack.size(), 0);
    }
}
