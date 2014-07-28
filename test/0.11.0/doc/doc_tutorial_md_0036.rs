fn main() {
    struct Point { x: f64, y: f64 }
    enum Direction { North, East, South, West }
    fn point_from_direction(dir: Direction) -> Point {
        match dir {
            North => Point { x:  0.0, y:  1.0 },
            East  => Point { x:  1.0, y:  0.0 },
            South => Point { x:  0.0, y: -1.0 },
            West  => Point { x: -1.0, y:  0.0 }
        }
    }
}