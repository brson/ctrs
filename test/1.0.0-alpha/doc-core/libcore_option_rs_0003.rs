fn main() {
    enum Kingdom { Plant(uint, &'static str), Animal(uint, &'static str) }
    
    // A list of data to search through.
    let all_the_big_things = [
        Kingdom::Plant(250, "redwood"),
        Kingdom::Plant(230, "noble fir"),
        Kingdom::Plant(229, "sugar pine"),
        Kingdom::Animal(25, "blue whale"),
        Kingdom::Animal(19, "fin whale"),
        Kingdom::Animal(15, "north pacific right whale"),
    ];
    
    // We're going to search for the name of the biggest animal,
    // but to start with we've just got `None`.
    let mut name_of_biggest_animal = None;
    let mut size_of_biggest_animal = 0;
    for big_thing in all_the_big_things.iter() {
        match *big_thing {
            Kingdom::Animal(size, name) if size > size_of_biggest_animal => {
                // Now we've found the name of some big animal
                size_of_biggest_animal = size;
                name_of_biggest_animal = Some(name);
            }
            Kingdom::Animal(..) | Kingdom::Plant(..) => ()
        }
    }
    
    match name_of_biggest_animal {
        Some(name) => println!("the biggest animal is {}", name),
        None => println!("there are no animals :(")
    }
}
