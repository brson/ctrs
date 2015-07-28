fn main() {
    static mut N: i32 = 5;

    

    unsafe {

        N += 1;

    

        println!("N: {}", N);

    }

}
