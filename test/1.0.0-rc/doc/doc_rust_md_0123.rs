fn main() {
    fn add(x: int, y: int) -> int {
      return x + y;
    }
    
    let mut x = add(5,7);
    
    type Binop<'a> = |int,int|: 'a -> int;
    let bo: Binop = add;
    x = bo(5,7);
}