fn main() {}

macro_rules! b {

    (    ($e     :expr) -> ($p     :pat) else $err     :stmt ;

      $( ($e_rest:expr) -> ($p_rest:pat) else $err_rest:stmt ; )*

      binds  $( $bind_res:ident ),*

    )

=> (0) }

