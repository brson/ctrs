fn main() {



macro_rules! biased_match_rec {

    // Handle the first layer

    (   ($e     :expr) -> ($p     :pat) else $err     :stmt ;

     $( ($e_rest:expr) -> ($p_rest:pat) else $err_rest:stmt ; )*

     binds $( $bind_res:ident ),*

    ) => (

        match $e {

            $p => {

                // Recursively handle the next layer

                biased_match_rec!($( ($e_rest) -> ($p_rest) else $err_rest ; )*

                                  binds $( $bind_res ),*

                )

            }

            _ => { $err }

        }

    );

    // Produce the requested values

    ( binds $( $bind_res:ident ),* ) => ( ($( $bind_res ),*) )

}



// Wrap the whole thing in a `let`.

macro_rules! biased_match {

    // special case: `let (x) = ...` is illegal, so use `let x = ...` instead

    ( $( ($e:expr) -> ($p:pat) else $err:stmt ; )*

      binds $bind_res:ident

    ) => (

        let $bind_res = biased_match_rec!(

            $( ($e) -> ($p) else $err ; )*

            binds $bind_res

        );

    );

    // more than one name: use a tuple

    ( $( ($e:expr) -> ($p:pat) else $err:stmt ; )*

      binds  $( $bind_res:ident ),*

    ) => (

        let ( $( $bind_res ),* ) = biased_match_rec!(

            $( ($e) -> ($p) else $err ; )*

            binds $( $bind_res ),*

        );

    )

}





enum T1 { Good1(T2, uint), Bad1}

struct T2 { body: T3 }

enum T3 { Good2(uint), Bad2}

fn f(x: T1) -> uint {

biased_match!(

    (x)       -> (T1::Good1(g1, val)) else { return 0 };

    (g1.body) -> (T3::Good2(result) ) else { panic!("Didn't get Good2") };

    binds val, result );

// complicated stuff goes here

return result + val;

}

}

