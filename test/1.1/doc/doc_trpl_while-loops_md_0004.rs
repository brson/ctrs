fn main() {
    let mut x = 5;

    

    loop {

        x += x - 3;

    

        println!("{}", x);

    

        if x % 5 == 0 { break; }

    }

}
