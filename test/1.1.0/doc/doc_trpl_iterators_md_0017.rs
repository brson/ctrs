fn main() {
    #![feature(step_by)]
    for i in (1..).step_by(5).take(5) {
        println!("{}", i);
    }
}
