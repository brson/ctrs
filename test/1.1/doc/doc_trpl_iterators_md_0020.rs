fn main() {
    (1..1000)

        .filter(|&x| x % 2 == 0)

        .filter(|&x| x % 3 == 0)

        .take(5)

        .collect::<Vec<i32>>();

}
