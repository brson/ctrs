fn factory() -> Box<Fn(i32) -> i32> {

    let num = 5;



    Box::new(move |x| x + num)

}

fn main() {

let f = factory();



let answer = f(1);

assert_eq!(6, answer);

}

