struct Foo<'a> {

    x: &'a int,

}



fn main() {

    let y = &5i;          // -+ y goes into scope

    let f = Foo { x: y }; // -+ f goes into scope

    // stuff              //  |

                          //  |

}                         // -+ f and y go out of scope

