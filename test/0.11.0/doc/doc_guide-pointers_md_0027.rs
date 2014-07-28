fn main() {
    fn box_succ(x: Box<int>) -> int { *x + 1 }
    
    fn rc_succ(x: std::rc::Rc<int>) -> int { *x + 1 }
}