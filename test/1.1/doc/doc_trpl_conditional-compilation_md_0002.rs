fn main() {
    #[cfg(any(not(unix), all(target_os="macos", target_arch = "powerpc")))]

    fn foo() {}

}
