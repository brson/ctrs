fn main() {
    fn my_err(s: &str) -> ! { panic!() }
    
    fn f(i: i32) -> i32 {
    if i == 42 {
    return 42;
    }
    else {
    my_err("Bad number!");
    }
    }
}
