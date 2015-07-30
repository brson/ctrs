#![allow(unused_must_use)]
macro_rules! write_html {
($w:expr, ) => (());

($w:expr, $e:tt) => (write!($w, "{}", $e));

($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
write!($w, "<{}>", stringify!($tag));
write_html!($w, $($inner)*);
write!($w, "</{}>", stringify!($tag));
write_html!($w, $($rest)*);
}};
}

fn main() {
  // FIXME(#21826)
use std::fmt::Write;
let mut out = String::new();

write_html!(&mut out,
html[
head[title["Macros guide"]]
body[h1["Macros are the best!"]]
]);

assert_eq!(out,
"<html><head><title>Macros guide</title></head>\
<body><h1>Macros are the best!</h1></body></html>");
}
