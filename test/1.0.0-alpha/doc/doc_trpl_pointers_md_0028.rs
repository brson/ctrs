fn main() {
    use std::boxed::Box;

    use std::rc::Rc;

    

    fn succ(x: &int) -> int { *x + 1 }

    

    let ref_x = &5i;

    let box_x = Box::new(5i);

    let rc_x  = Rc::new(5i);

    

    succ(ref_x);

    succ(&*box_x);

    succ(&*rc_x);

}
