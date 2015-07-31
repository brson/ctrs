fn main() {
    let my_directory = if cfg!(windows) {
    "windows-specific-directory"
    } else {
    "unix-directory"
    };
}
