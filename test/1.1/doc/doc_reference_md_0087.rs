fn main() {
    let x = &3;

    let y = match *x { 0 => "zero", _ => "some" };

    let z = match x { &0 => "zero", _ => "some" };

    

    assert_eq!(y, z);

}
