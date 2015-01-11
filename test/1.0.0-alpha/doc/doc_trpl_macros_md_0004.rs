macro_rules! biased_match {

    // special case: `let (x) = ...` is illegal, so use `let x = ...` instead

    ( ($e:expr) -> ($p:pat) else $err:stmt ;

      binds $bind_res:ident

    ) => (

        let $bind_res = match $e {

            $p => ( $bind_res ),

            _ => { $err }

        };

    );

    // more than one name; use a tuple

    ( ($e:expr) -> ($p:pat) else $err:stmt ;

      binds $( $bind_res:ident ),*

    ) => (

        let ( $( $bind_res ),* ) = match $e {

            $p => ( $( $bind_res ),* ),

            _ => { $err }

        };

    )

}



enum T1 { Good1(T2, uint), Bad1}

struct T2 { body: T3 }

enum T3 { Good2(uint), Bad2}

fn f(x: T1) -> uint {

biased_match!((x)       -> (T1::Good1(g1, val)) else { return 0 };

              binds g1, val );

biased_match!((g1.body) -> (T3::Good2(result) )

                  else { panic!("Didn't get good_2") };

              binds result );

// complicated stuff goes here

return result + val;

}

fn main() {}

