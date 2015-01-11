fn main() {
    struct Point(int, int);

    let p = Point(10, 11);

    let px: int = match p { Point(x, _) => x };

}
