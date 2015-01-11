fn main() {
    trait Shape { fn area() -> f64; }

    trait Circle : Shape { fn radius() -> f64; }

}
