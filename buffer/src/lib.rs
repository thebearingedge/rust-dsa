#[derive(Debug, PartialEq)]
pub struct Buffer<T> {
    cap: usize,
    ptr: std::ptr::NonNull<T>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Buffer<T> {
    pub fn new() -> Self {
        Self {
            cap: 0,
            ptr: std::ptr::NonNull::dangling(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn with_capacity(size: usize) -> Self {
        let layout = std::alloc::Layout::array::<T>(size).unwrap();
        let raw = unsafe { std::alloc::alloc(layout) as *mut T };
        let ptr = std::ptr::NonNull::new(raw).unwrap();
        Self {
            cap: size,
            ptr,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn grow(&mut self) {
        if self.cap == 0 {
            let layout = std::alloc::Layout::array::<T>(4).unwrap();
            let raw = unsafe { std::alloc::alloc(layout) as *mut T };
            self.ptr = std::ptr::NonNull::new(raw).unwrap();
            self.cap = 4;
        } else {
            let layout = std::alloc::Layout::array::<T>(self.cap).unwrap();
            assert!(layout.size() <= isize::MAX as usize, "allocation too large");
            let raw = unsafe {
                std::alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, layout.size() * 2)
                    as *mut T
            };
            self.ptr = std::ptr::NonNull::new(raw).unwrap();
            self.cap *= 2;
        }
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub unsafe fn read(&self, index: usize) -> T {
        self.ptr.as_ptr().add(index).read()
    }

    pub unsafe fn write(&self, index: usize, elem: T) {
        self.ptr.as_ptr().add(index).write(elem);
    }

    pub fn copy(&mut self, src: usize, dst: usize, len: usize) {
        assert!(
            src < self.cap,
            "source index out of bounds: the capacity is {} but the index is {}",
            self.cap,
            src
        );
        assert!(
            dst + len < self.cap,
            "destination index out of bounds: the capacity is {} but last index is {}",
            self.cap,
            dst + len
        );
        unsafe {
            self.ptr
                .as_ptr()
                .add(src)
                .copy_to_nonoverlapping(self.ptr.as_ptr().add(dst), len);
        }
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            unsafe {
                let slice = std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.cap);
                std::ptr::drop_in_place(slice);
                let layout = std::alloc::Layout::array::<T>(self.cap).unwrap();
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
        let buf = Buffer::<i32>::new();
        assert_eq!(buf.capacity(), 0);
    }

    #[test]
    fn test_with_capacity() {
        let buf = Buffer::<i32>::with_capacity(4);
        assert_eq!(buf.capacity(), 4);
    }

    #[test]
    fn test_grow() {
        let mut buf = Buffer::<i32>::new();
        buf.grow();
        assert_eq!(buf.capacity(), 4);
        buf.grow();
        assert_eq!(buf.capacity(), 8);
    }
}
