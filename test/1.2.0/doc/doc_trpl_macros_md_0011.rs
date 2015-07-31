macro_rules! o_O {
(
$(
$x:expr; [ $( $y:expr ),* ]
);*
) => {
&[ $($( $x + $y ),*),* ]
}
}

fn main() {
let a: &[i32]
= o_O!(10; [1, 2, 3];
20; [4, 5, 6]);

assert_eq!(a, [11, 12, 13, 24, 25, 26]);
}
