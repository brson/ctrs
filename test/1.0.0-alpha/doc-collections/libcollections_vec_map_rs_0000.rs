fn main() {
    use std::collections::VecMap;
    
    let mut months = VecMap::new();
    months.insert(1, "Jan");
    months.insert(2, "Feb");
    months.insert(3, "Mar");
    
    if !months.contains_key(&12) {
        println!("The end is near!");
    }
    
    assert_eq!(months.get(&1), Some(&"Jan"));
    
    match months.get_mut(&3) {
        Some(value) => *value = "Venus",
        None => (),
    }
    
    assert_eq!(months.get(&3), Some(&"Venus"));
    
    // Print out all months
    for (key, value) in months.iter() {
        println!("month {} is {}", key, value);
    }
    
    months.clear();
    assert!(months.is_empty());
}
