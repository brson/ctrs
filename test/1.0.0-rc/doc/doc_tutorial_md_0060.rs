fn main() {
    let r = box 13i;
    let mut s = r; // box becomes mutable
    *s += 1;
    let t = s; // box becomes immutable
}