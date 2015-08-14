fn main() {
    enum Message {
    Write(String),
    }
    let m = Message::Write("Hello, world".to_string());
}
