fn main() {
    struct Point3d {

        x: i32,

        y: i32,

        z: i32,

    }

    

    let mut point = Point3d { x: 0, y: 0, z: 0 };

    point = Point3d { y: 1, .. point };

}
