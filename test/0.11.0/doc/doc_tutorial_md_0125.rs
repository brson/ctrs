fn main() {
    trait Shape { fn area(&self) -> f64; }
    trait Circle : Shape { fn radius(&self) -> f64; }
}