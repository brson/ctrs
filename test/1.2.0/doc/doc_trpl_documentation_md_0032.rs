#[macro_export]
macro_rules! panic_unless {
($condition:expr, $($rest:expr),+) => ({ if ! $condition { panic!($($rest),+); } });
}
fn main() {}
