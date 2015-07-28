fn main() {
    let greater_than_forty_two = (0..100)

                                 .find(|x| *x > 42);

    

    match greater_than_forty_two {

        Some(_) => println!("We got some numbers!"),

        None => println!("No numbers found :("),

    }

}
