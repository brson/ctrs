fn main() {
    fn my_err(s: &str) -> ! { fail!() }
    
    fn f(i: int) -> int {
       if i == 42 {
         return 42;
       }
       else {
         my_err("Bad number!");
       }
    }
}