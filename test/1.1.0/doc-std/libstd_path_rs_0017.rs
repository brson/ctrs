fn main() {
    use std::path::Path;        let path = Path::new("/foo/bar");    let foo = path.parent().unwrap();        assert!(foo == Path::new("/foo"));        let root = foo.parent().unwrap();        assert!(root == Path::new("/"));    assert!(root.parent() == None);}
