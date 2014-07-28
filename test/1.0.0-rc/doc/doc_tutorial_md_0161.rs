fn main() {
    #![allow(unused_attribute)]
    // `world.rs`
    #![crate_id = "world#0.42"]
    
    mod secret_module_to_make_this_test_run {
    pub fn explore() -> &'static str { "world" }
    }
}