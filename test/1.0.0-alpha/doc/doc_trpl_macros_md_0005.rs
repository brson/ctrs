fn main() {}

macro_rules! b {

    ( $( ($e:expr) -> ($p:pat) else $err:stmt ; )*

      binds $( $bind_res:ident ),*

    )

=> (0) }

