fn main() {
    use std::uint;
    
    let v = vec!(1u, 2u);
    let res: Result<Vec<uint>, &'static str> = v.iter().map(|&x: &uint|
        if x == uint::MAX { Err("Overflow!") }
        else { Ok(x + 1) }
    ).collect();
    assert!(res == Ok(vec!(2u, 3u)));
}
