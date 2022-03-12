use ::std::mem;

/// A buffer type, that is is supposed to change size on use with a C-function.
#[repr(transparent)]
pub struct DynBuffer {
    inner: Vec<u8>,
}

impl DynBuffer {
    /// Creates a new empty [`DynBuffer`].
    pub fn new() -> Self {
        Self {
            inner: Vec::new()
        }
    }

    /// Creates a new [`DynBuffer`] with a given capacity. This helps if it needs to be 
    /// differernt sized multiple times.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity)
        }
    }

    /// Returns the number of elements of the buffer as T.
    pub fn len<T>(&self) -> usize {
        self.inner.len() / mem::size_of::<T>()
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

