use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    shared_map.borrow_mut().insert("africa", 92388i);
    shared_map.borrow_mut().insert("kyoto", 11837i);
    shared_map.borrow_mut().insert("piccadilly", 11826i);
    shared_map.borrow_mut().insert("marbles", 38i);
}
