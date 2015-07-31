use std::ops::{Deref, DerefMut};

struct DerefMutExample<T> {
    value: T
}

impl<T> Deref for DerefMutExample<T> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        &self.value
    }
}

impl<T> DerefMut for DerefMutExample<T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        &mut self.value
    }
}

fn main() {
    let mut x = DerefMutExample { value: 'a' };
    *x = 'b';
    assert_eq!('b', *x);
}
