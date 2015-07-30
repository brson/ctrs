fn main() {
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point3d { x: 0, y: 0, z: 0 };
    let point = Point3d { z: 1, x: 2, .. origin };
}
