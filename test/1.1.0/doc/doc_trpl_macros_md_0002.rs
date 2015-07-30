macro_rules! vec {
( $( $x:expr ),* ) => {
{
let mut temp_vec = Vec::new();
$(
temp_vec.push($x);
)*
temp_vec
}
};
}
fn main() {
    assert_eq!(vec![1,2,3], [1, 2, 3]);
}
