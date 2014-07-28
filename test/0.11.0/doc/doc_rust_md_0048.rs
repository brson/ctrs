fn main() {
    type Surface = int;
    type BoundingBox = int;
    trait Shape {
        fn draw(&self, Surface);
        fn bounding_box(&self) -> BoundingBox;
    }
}