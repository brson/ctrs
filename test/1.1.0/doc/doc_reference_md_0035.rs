fn main() {
    type Surface = i32;
    type BoundingBox = i32;
    trait Shape {
    fn draw(&self, Surface);
    fn bounding_box(&self) -> BoundingBox;
    }
}
