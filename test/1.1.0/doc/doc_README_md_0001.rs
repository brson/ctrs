fn main() {
    pandoc --from=markdown --to=html5 --number-sections -o reference.html reference.md
}
