fn main() {
    let mut x = 5u;

    let mut done = false;

    

    while !done {

        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 { done = true; }

    }

}
