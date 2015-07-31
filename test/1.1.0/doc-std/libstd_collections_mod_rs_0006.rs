fn main() {
    use std::collections::btree_map::BTreeMap;
    
    // A client of the bar. They have an id and a blood alcohol level.
    struct Person { id: u32, blood_alcohol: f32 }
    
    // All the orders made to the bar, by client id.
    let orders = vec![1,2,1,2,3,4,1,2,2,3,4,1,1,1];
    
    // Our clients.
    let mut blood_alcohol = BTreeMap::new();
    
    for id in orders.into_iter() {
        // If this is the first time we've seen this customer, initialize them
        // with no blood alcohol. Otherwise, just retrieve them.
        let person = blood_alcohol.entry(id).or_insert(Person{id: id, blood_alcohol: 0.0});
    
        // Reduce their blood alcohol level. It takes time to order and drink a beer!
        person.blood_alcohol *= 0.9;
    
        // Check if they're sober enough to have another beer.
        if person.blood_alcohol > 0.3 {
            // Too drunk... for now.
            println!("Sorry {}, I have to cut you off", person.id);
        } else {
            // Have another!
            person.blood_alcohol += 0.1;
        }
    }
}
