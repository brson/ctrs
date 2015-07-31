fn main() {
    let a = [1, 4, 2, 3, 8, 9, 6];
    let sum: i32 = a.iter()
                    .map(|x| *x)
                    .inspect(|&x| println!("filtering {}", x))
                    .filter(|&x| x % 2 == 0)
                    .inspect(|&x| println!("{} made it through", x))
                    .fold(0, |sum, i| sum + i);
    println!("{}", sum);
}
