fn main() {
    use std::collections::HashSet;
    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Viking<'a> {
    name: &'a str,
    power: usize,
    }
    
    let mut vikings = HashSet::new();
    
    vikings.insert(Viking { name: "Einar", power: 9 });
    vikings.insert(Viking { name: "Einar", power: 9 });
    vikings.insert(Viking { name: "Olaf", power: 4 });
    vikings.insert(Viking { name: "Harald", power: 8 });
    
    // Use derived implementation to print the vikings.
    for x in &vikings {
    println!("{:?}", x);
    }
}
