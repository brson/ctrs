fn main() {
    enum Message {
    Write(String),
    }
    fn foo(x: String) -> Message {
    Message::Write(x)
    }
    
    let x = foo("Hello, world".to_string());
}
