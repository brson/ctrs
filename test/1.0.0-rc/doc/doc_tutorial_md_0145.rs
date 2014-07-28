fn main() {
    mod workaround {
    pub fn some_parent_item(){ println!("...") }
    mod foo {
    use super::some_parent_item;
    use self::some_child_module::some_item;
    pub fn bar() { some_parent_item(); some_item() }
    pub mod some_child_module { pub fn some_item() {} }
    }
    }
}