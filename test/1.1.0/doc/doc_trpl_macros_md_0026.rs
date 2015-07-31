fn main() {
    macro_rules! bct {
        // cmd 0:  d ... => ...
        (0, $($ps:tt),* ; $_d:tt)
            => (bct!($($ps),*, 0 ; ));
        (0, $($ps:tt),* ; $_d:tt, $($ds:tt),*)
            => (bct!($($ps),*, 0 ; $($ds),*));
    
        // cmd 1p:  1 ... => 1 ... p
        (1, $p:tt, $($ps:tt),* ; 1)
            => (bct!($($ps),*, 1, $p ; 1, $p));
        (1, $p:tt, $($ps:tt),* ; 1, $($ds:tt),*)
            => (bct!($($ps),*, 1, $p ; 1, $($ds),*, $p));
    
        // cmd 1p:  0 ... => 0 ...
        (1, $p:tt, $($ps:tt),* ; $($ds:tt),*)
            => (bct!($($ps),*, 1, $p ; $($ds),*));
    
        // halt on empty data string
        ( $($ps:tt),* ; )
            => (());
    }
}
