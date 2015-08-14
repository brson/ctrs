fn main() {
    use std::cell::RefCell;
    
    let x = RefCell::new(vec![1,2,3,4]);
    {
        println!("{:?}", *x.borrow())
    }
    
    {
        let mut my_ref = x.borrow_mut();
        my_ref.push(1);
    }
}
