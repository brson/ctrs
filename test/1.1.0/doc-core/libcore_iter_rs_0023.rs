fn main() {
    let a = [1, 2, 3, 4, 5];    assert_eq!(a.iter().fold(0, |acc, &item| acc + item), 15);}
