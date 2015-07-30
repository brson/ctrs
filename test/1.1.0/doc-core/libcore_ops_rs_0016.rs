use std::ops::Deref;

struct DerefExample<T> {
value: T
}

impl<T> Deref for DerefExample<T> {
type Target = T;

fn deref<'a>(&'a self) -> &'a T {
&self.value
}
}

fn main() {
let x = DerefExample { value: 'a' };
assert_eq!('a', *x);
}
