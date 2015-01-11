fn main() {
    fn ten_times<F>(f: F) where F: Fn(int) {

        let mut i = 0;

        while i < 10 {

            f(i);

            i += 1;

        }

    }

    

    ten_times(|j| println!("hello, {}", j));

}
