fn main() {
    use std::path::Path;
    
    let path = Path::new("/foo/bar");
    let parent = path.parent().unwrap();
    assert_eq!(parent, Path::new("/foo"));
    
    let grand_parent = parent.parent().unwrap();
    assert_eq!(grand_parent, Path::new("/"));
    assert_eq!(grand_parent.parent(), None);
}
