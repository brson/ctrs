fn main() {
    let x = 2i;
    
    let message = match x {
      0 | 1  => "not many",
      2 .. 9 => "a few",
      _      => "lots"
    };
}