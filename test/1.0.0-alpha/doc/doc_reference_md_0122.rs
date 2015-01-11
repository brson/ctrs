fn main() {
    let maybe_digit = Some(0);

    fn process_digit(i: int) { }

    fn process_other(i: int) { }

    

    let message = match maybe_digit {

      Some(x) if x < 10 => process_digit(x),

      Some(x) => process_other(x),

      None => panic!()

    };

}
