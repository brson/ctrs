fn main() {
    use std::cell::Cell;
    
    struct Rc<T> {
        ptr: *mut RcBox<T>
    }
    
    struct RcBox<T> {
        value: T,
        refcount: Cell<usize>
    }
    
    impl<T> Clone for Rc<T> {
        fn clone(&self) -> Rc<T> {
            unsafe {
                (*self.ptr).refcount.set((*self.ptr).refcount.get() + 1);
                Rc { ptr: self.ptr }
            }
        }
    }
}
