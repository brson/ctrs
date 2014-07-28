fn is_symmetric(list: &[uint]) -> bool {
    match list {
        [] | [_]                   => true,
        [x, ..inside, y] if x == y => is_symmetric(inside),
        _                          => false
    }
}

fn main() {
    let sym     = &[0, 1, 4, 2, 4, 1, 0];
    let not_sym = &[0, 1, 7, 2, 4, 1, 0];
    assert!(is_symmetric(sym));
    assert!(!is_symmetric(not_sym));
}
