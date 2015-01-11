fn main() {
    use std::boxed::Box;
    use std::mem;
    
    unsafe {
        let my_num: Box<int> = Box::new(10);
        let my_num: *const int = mem::transmute(my_num);
        let my_speed: Box<int> = Box::new(88);
        let my_speed: *mut int = mem::transmute(my_speed);
    
        // By taking ownership of the original `Box<T>` though
        // we are obligated to transmute it back later to be destroyed.
        drop(mem::transmute::<_, Box<int>>(my_speed));
        drop(mem::transmute::<_, Box<int>>(my_num));
    }
}
