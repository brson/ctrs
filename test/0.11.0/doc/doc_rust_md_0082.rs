fn main() {
    struct Point3d { x: int, y: int, z: int }
    let base = Point3d {x: 1, y: 2, z: 3};
    Point3d {y: 0, z: 10, .. base};
}