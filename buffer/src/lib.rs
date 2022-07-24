use std::ptr::NonNull;

#[derive(Debug, PartialEq)]
pub struct Buffer<T> {
    len: usize,
    capacity: usize,
    ptr: NonNull<T>,
}

impl<T> Buffer<T> {
    pub fn empty() -> Self {
        Self {
            len: 0,
            capacity: 0,
            ptr: NonNull::dangling(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
        let raw = unsafe { std::alloc::alloc(layout) as *mut T };
        let ptr = std::ptr::NonNull::new(raw).unwrap();
        Self {
            ptr,
            len: 0,
            capacity,
        }
    }

    pub fn is_full(&self) -> bool {
        self.len == self.capacity
    }

    pub fn grow(&mut self) {
        let layout = std::alloc::Layout::array::<T>(self.capacity * 2).unwrap();
        let raw = unsafe {
            std::alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, layout.size()) as *mut T
        };
        self.ptr = std::ptr::NonNull::new(raw).unwrap();
        self.capacity *= 2;
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        if self.ptr == std::ptr::NonNull::dangling() {
            return;
        }
        unsafe {
            let slice = std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.capacity);
            std::ptr::drop_in_place(slice);
            let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
            std::alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
