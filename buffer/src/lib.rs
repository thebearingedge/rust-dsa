#[derive(Debug, PartialEq)]
pub struct Buffer<T> {
    cap: usize,
    ptr: std::ptr::NonNull<T>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Buffer<T> {
    pub fn empty() -> Self {
        Self {
            cap: 0,
            ptr: std::ptr::NonNull::dangling(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
        let raw = unsafe { std::alloc::alloc(layout) as *mut T };
        let ptr = std::ptr::NonNull::new(raw).unwrap();
        Self {
            ptr,
            cap: capacity,
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
