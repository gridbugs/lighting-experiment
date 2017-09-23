pub struct VecPool<T> {
    vecs: Vec<Vec<T>>,
}

impl<T> VecPool<T> {
    pub fn new() -> Self {
        Self {
            vecs: Vec::new(),
        }
    }

    pub fn alloc(&mut self) -> Vec<T> {
        self.vecs.pop().unwrap_or_else(|| Vec::new())
    }

    pub fn free(&mut self, v: Vec<T>) {
        self.vecs.push(v);
    }
}
