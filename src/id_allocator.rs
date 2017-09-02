use num::Integer;

pub struct IdAllocator<T: Integer + Copy>  {
    next: T,
    free_list: Vec<T>,
}

impl<T: Integer + Copy> IdAllocator<T> {
    pub fn new() -> Self {
        Self {
            next: T::zero(),
            free_list: Vec::new(),
        }
    }

    pub fn allocate(&mut self) -> T {
        if let Some(id) = self.free_list.pop() {
            id
        } else {
            let id = self.next;
            self.next = self.next + T::one();
            id
        }
    }

    pub fn peek(&self) -> T {
        self.next
    }

    pub fn free(&mut self, id: T) {
        self.free_list.push(id);
    }
}
