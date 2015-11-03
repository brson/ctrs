fn main() {
    use std::hash::{Hash, SipHasher, Hasher};
    
    #[derive(Hash)]
    struct Person {
        id: u32,
        name: String,
        phone: u64,
    }
    
    let person1 = Person { id: 5, name: "Janet".to_string(), phone: 555_666_7777 };
    let person2 = Person { id: 5, name: "Bob".to_string(), phone: 555_666_7777 };
    
    assert!(hash(&person1) != hash(&person2));
    
    fn hash<T: Hash>(t: &T) -> u64 {
        let mut s = SipHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}
