fn main() {
    use std::collections::BTreeMap;        let mut map = BTreeMap::new();    map.insert(1, "a");    map.insert(2, "b");    map.insert(3, "c");        for (key, value) in map.iter() {        println!("{}: {}", key, value);    }        let (first_key, first_value) = map.iter().next().unwrap();    assert_eq!((*first_key, *first_value), (1, "a"));}
