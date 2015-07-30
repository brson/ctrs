fn main() {
    let x = ("colorless",  "green", "ideas", "sleep", "furiously");
    assert_eq!(x.3, "sleep");
    
    let v = (3, 3);
    let u = (1, -5);
    assert_eq!(v.0 * u.0 + v.1 * u.1, -12);
}
