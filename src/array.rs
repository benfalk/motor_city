#[repr(C)]
pub struct Array<T> {
    size: usize,
    capacity: usize,
    items: *mut *mut T,
}

impl<T> From<Vec<T>> for Array<T> {
    fn from(mut items: Vec<T>) -> Self {
        let items: Vec<*mut T> = items
            .drain(..)
            .map(|item| Box::into_raw(Box::new(item)))
            .collect();

        let (items, size, capacity) = items.into_raw_parts();

        Self {
            size,
            capacity,
            items,
        }
    }
}

impl <T> Array<T> {
    pub fn free(self) {
        unsafe {
            Vec::from_raw_parts(
                self.items, 
                self.size,
                self.capacity
            );
        }
    }
}
