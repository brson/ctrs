fn main() {
    use std::u32;
    
    let v = vec!(1, 2);
    let res: Result<Vec<u32>, &'static str> = v.iter().map(|&x: &u32|
        if x == u32::MAX { Err("Overflow!") }
        else { Ok(x + 1) }
    ).collect();
    assert!(res == Ok(vec!(2, 3)));
}
