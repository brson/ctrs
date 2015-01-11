fn main() {
    let x = ("colorless",  "green", "ideas", "sleep", "furiously");
    assert_eq!(x.3, "sleep");
    
    let v = (3i, 3i);
    let u = (1i, -5i);
    assert_eq!(v.0 * u.0 + v.1 * u.1, -12i);
}
