fn main() {
    fn foo() -> std::io::Result<()> {    use std::fs;        let mut perms = try!(fs::metadata("foo.txt")).permissions();    perms.set_readonly(true);    try!(fs::set_permissions("foo.txt", perms));    Ok(())    }}
