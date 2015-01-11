enum T1 { Good1(T2, uint), Bad1}

struct T2 { body: T3 }

enum T3 { Good2(uint), Bad2}

fn f(x: T1) -> uint {

match x {

    T1::Good1(g1, val) => {

        match g1.body {

            T3::Good2(result) => {

                // complicated stuff goes here

                return result + val;

            },

            _ => panic!("Didn't get good_2")

        }

    }

    _ => return 0 // default value

}

}

fn main() {}

