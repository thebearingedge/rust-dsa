#[derive(Debug, PartialEq)]
pub struct Vector<T> {
    ptr: std::ptr::NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Self {
            ptr: std::ptr::NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        if self.capacity == 0 {
            let layout = std::alloc::Layout::array::<T>(4).unwrap();
            let ptr = unsafe { std::alloc::alloc(layout) as *mut T };
            self.ptr = std::ptr::NonNull::new(ptr).unwrap();
            self.capacity = 4;
        } else if self.len == self.capacity {
            let layout = std::alloc::Layout::array::<T>(self.capacity * 2).unwrap();
            let ptr = unsafe {
                std::alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, layout.size()) as *mut T
            };
            self.ptr = std::ptr::NonNull::new(ptr).unwrap();
            self.capacity *= 2;
        }
        unsafe { self.ptr.as_ptr().add(self.len).write(element) };
        self.len += 1;
    }

    pub fn at(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        Some(unsafe { &*self.ptr.as_ptr().add(index) })
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        if self.ptr != std::ptr::NonNull::dangling() {
            unsafe {
                let slice = std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.capacity);
                std::ptr::drop_in_place(slice);
                let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
                std::alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vector = Vector::<i32>::new();
        assert_eq!(vector.capacity(), 0);
    }

    #[test]
    fn test_push() {
        let mut vector = Vector::<char>::new();
        vector.push('a');
        vector.push('b');
        assert_eq!(vector.len(), 2);
        assert_eq!(vector.capacity(), 4);
        vector.push('c');
        vector.push('d');
        vector.push('e');
        assert_eq!(vector.len(), 5);
        assert_eq!(vector.capacity(), 8);
    }
}
