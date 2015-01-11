fn main() {
    use std::uint;
    
    let v = vec!(1u, 2u);
    let res: Option<Vec<uint>> = v.iter().map(|&x: &uint|
        if x == uint::MAX { None }
        else { Some(x + 1) }
    ).collect();
    assert!(res == Some(vec!(2u, 3u)));
}
