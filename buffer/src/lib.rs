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
            ptr,
            cap: size,
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
            assert!(layout.size() <= isize::MAX as usize, "allocation to large");
            let raw = unsafe {
                std::alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, layout.size() * 2)
                    as *mut T
            };
            self.ptr = std::ptr::NonNull::new(raw).unwrap();
            self.cap *= 2;
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

impl<T> std::ops::Deref for Buffer<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.cap) }
    }
}

impl<T> std::ops::DerefMut for Buffer<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.cap) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let buf = Buffer::<i32>::new();
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn test_with_capacity() {
        let buf = Buffer::<i32>::with_capacity(4);
        assert_eq!(buf.len(), 4);
    }

    #[test]
    fn test_grow() {
        let mut buf = Buffer::<i32>::new();
        buf.grow();
        assert_eq!(buf.len(), 4);
        buf.grow();
        assert_eq!(buf.len(), 8);
    }

    #[test]
    fn test_deref() {
        let mut buf = Buffer::<i32>::with_capacity(4);
        buf[0] = 42;
        assert_eq!(buf[0], 42);
    }
}
