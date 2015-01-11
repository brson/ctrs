fn main() {
    type Foo = int;

    fn bar(f: Foo) { }

    let a = 0;

    let b = 0;

    let c = 0;

    

    let v: &[Foo] = &[a, b, c];

    

    for e in v.iter() {

        bar(*e);

    }

}
