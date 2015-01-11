fn main() {
    use std::rc::Rc;

    

    fn box_succ(x: Box<int>) -> int { *x + 1 }

    

    fn rc_succ(x: Rc<int>) -> int { *x + 1 }

}
