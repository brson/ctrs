fn main() {
    use option::Option;
    use mem;
    
    let i: int = mem::transmute(Option(0));
}
