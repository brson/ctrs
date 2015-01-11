fn main() {
    use std::collections::HashSet;
    #[derive(Hash, Eq, PartialEq, Show)]
    struct Viking<'a> {
        name: &'a str,
        power: uint,
    }
    
    let mut vikings = HashSet::new();
    
    vikings.insert(Viking { name: "Einar", power: 9u });
    vikings.insert(Viking { name: "Einar", power: 9u });
    vikings.insert(Viking { name: "Olaf", power: 4u });
    vikings.insert(Viking { name: "Harald", power: 8u });
    
    // Use derived implementation to print the vikings.
    for x in vikings.iter() {
        println!("{:?}", x);
    }
}
