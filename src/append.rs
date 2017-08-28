use std::collections::VecDeque;

pub trait Append<T> {
    fn append(&mut self, value: T);
}

impl<T> Append<T> for Vec<T> {
    fn append(&mut self, value: T) {
        self.push(value);
    }
}

impl<T> Append<T> for VecDeque<T> {
    fn append(&mut self, value: T) {
        self.push_back(value);
    }
}
