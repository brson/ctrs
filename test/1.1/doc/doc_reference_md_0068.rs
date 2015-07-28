fn main() {
    struct Point3d { x: i32, y: i32, z: i32 }

    let base = Point3d {x: 1, y: 2, z: 3};

    Point3d {y: 0, z: 10, .. base};

}
