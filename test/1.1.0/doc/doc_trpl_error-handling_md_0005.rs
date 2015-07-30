use Event::NewRelease;

enum Event {
NewRelease,
}

fn probability(_: &Event) -> f64 {
// real implementation would be more complex, of course
0.95
}

fn descriptive_probability(event: Event) -> &'static str {
match probability(&event) {
1.00 => "certain",
0.00 => "impossible",
0.00 ... 0.25 => "very unlikely",
0.25 ... 0.50 => "unlikely",
0.50 ... 0.75 => "likely",
0.75 ... 1.00 => "very likely",
_ => unreachable!()
}
}

fn main() {
println!("{}", descriptive_probability(NewRelease));
}
