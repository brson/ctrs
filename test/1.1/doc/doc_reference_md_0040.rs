fn main() {
    type Surface = i32;

    trait Shape { fn draw(&self, Surface); }

    fn draw_twice<T: Shape>(surface: Surface, sh: T) {

        sh.draw(surface);

        sh.draw(surface);

    }

}
