use std::cell::RefCell;

struct Graph {
    edges: Vec<(uint, uint)>,
    span_tree_cache: RefCell<Option<Vec<(uint, uint)>>>
}

impl Graph {
    fn minimum_spanning_tree(&self) -> Vec<(uint, uint)> {
        // Create a new scope to contain the lifetime of the
        // dynamic borrow
        {
            // Take a reference to the inside of cache cell
            let mut cache = self.span_tree_cache.borrow_mut();
            if cache.is_some() {
                return cache.as_ref().unwrap().clone();
            }

            let span_tree = self.calc_span_tree();
            *cache = Some(span_tree);
        }

        // Recursive call to return the just-cached value.
        // Note that if we had not let the previous borrow
        // of the cache fall out of scope then the subsequent
        // recursive borrow would cause a dynamic task panic.
        // This is the major hazard of using `RefCell`.
        self.minimum_spanning_tree()
    }
  fn calc_span_tree(&self) -> Vec<(uint, uint)> { vec![] }
}
fn main() { }
