fn main() {
    let x: Option<i32> = None;

    

    match x {

        Some(_) => unreachable!(),

        None => println!("I know x is None!"),

    }

}
