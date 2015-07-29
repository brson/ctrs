fn main() {
    use std::borrow::Cow;        let cow: Cow<[_]> = Cow::Owned(vec![1, 2, 3]);        let hello = cow.into_owned();        assert_eq!(vec![1, 2, 3], hello);}
