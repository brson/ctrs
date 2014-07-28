mod farm {
pub type Chicken = int;
struct Human(int);
impl Human { pub fn rest(&self) { } }
pub fn make_me_a_farm() -> Farm { Farm { chickens: vec![], farmer: Human(0) } }
    pub struct Farm {
        chickens: Vec<Chicken>,
        pub farmer: Human
    }

    impl Farm {
        fn feed_chickens(&self) { /* ... */ }
        pub fn add_chicken(&self, c: Chicken) { /* ... */ }
    }

    pub fn feed_animals(farm: &Farm) {
        farm.feed_chickens();
    }
}

fn main() {
    let f = make_me_a_farm();
    f.add_chicken(make_me_a_chicken());
    farm::feed_animals(&f);
    f.farmer.rest();

    // This wouldn't compile because both are private:
    // `f.feed_chickens();`
    // `let chicken_counter = f.chickens.len();`
}
fn make_me_a_farm() -> farm::Farm { farm::make_me_a_farm() }
fn make_me_a_chicken() -> farm::Chicken { 0 }
