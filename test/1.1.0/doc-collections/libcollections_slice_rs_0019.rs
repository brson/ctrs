fn main() {
    let mut v = [1, 2, 3, 4, 5, 6];
    
    // scoped to restrict the lifetime of the borrows
    {
       let (left, right) = v.split_at_mut(0);
       assert!(left == []);
       assert!(right == [1, 2, 3, 4, 5, 6]);
    }
    
    {
        let (left, right) = v.split_at_mut(2);
        assert!(left == [1, 2]);
        assert!(right == [3, 4, 5, 6]);
    }
    
    {
        let (left, right) = v.split_at_mut(6);
        assert!(left == [1, 2, 3, 4, 5, 6]);
        assert!(right == []);
    }
}
