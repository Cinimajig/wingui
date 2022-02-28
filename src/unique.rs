use ::std::{
    io,
    ptr::NonNull,
};

pub struct UniquePtr<T>(*mut T);

impl<T> UniquePtr<T> {
    pub fn from_raw(ptr: *mut T) -> io::Result<Self> {
            if ptr.is_null() {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Pointer is null."));
            }

            Ok(Self(ptr))
    }

    #[inline(always)]
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

impl<T> Drop for UniquePtr<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self.0);
        }
    }
}

