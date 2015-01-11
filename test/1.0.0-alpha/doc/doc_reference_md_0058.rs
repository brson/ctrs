fn main() {
    trait Shape { fn area(&self) -> f64; }

    trait Circle : Shape { fn radius(&self) -> f64; }

    fn radius_times_area<T: Circle>(c: T) -> f64 {

        // `c` is both a Circle and a Shape

        c.radius() * c.area()

    }

}
