fn main() {
    let my_number = 1i;
    match my_number {
      0     => println!("zero"),
      1 | 2 => println!("one or two"),
      3..10 => println!("three to ten"),
      _     => println!("something else")
    }
}