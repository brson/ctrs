fn main() {
    let expected = [1, 2, 3, 4, 5];    let actual: Vec<_> = expected.iter().cloned().collect();    assert_eq!(actual, expected);}
