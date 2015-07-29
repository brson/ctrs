fn main() {
    fn iter(&self) -> T           // where T implements Iterator<&U>
    fn iter_mut(&mut self) -> T   // where T implements Iterator<&mut U>
    fn into_iter(self) -> T       // where T implements Iterator<U>
}
