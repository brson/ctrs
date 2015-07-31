fn main() {
    let words = ["alpha", "beta", "gamma"];
    let merged: String = words.iter()
                              .flat_map(|s| s.chars())
                              .collect();
    assert_eq!(merged, "alphabetagamma");
}
