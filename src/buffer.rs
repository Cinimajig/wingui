use ::std::mem;

#[repr(transparent)]
pub struct DynBuffer {
    inner: Vec<u8>,
}

impl DynBuffer {
    pub fn new() -> Self {
        Self {
            inner: Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn as_dyn_ptr<T>(&mut self, count: usize) -> *mut T {
        self.inner.resize(mem::size_of::<T>() * count, 0);
        self.inner.as_mut_ptr().cast()
    }

    pub fn as_ref<T>(&self) -> &T {
        unsafe {
            mem::transmute(&self.inner)
        }
    } 
}

