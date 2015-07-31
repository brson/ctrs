fn main() {
    let mut scores = [7, 8, 9];
    for score in &mut scores[..] {
        *score += 1;
    }
}
