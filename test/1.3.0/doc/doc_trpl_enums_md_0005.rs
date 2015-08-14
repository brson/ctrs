fn main() {
    enum Message {
    Write(String),
    }
    
    let v = vec!["Hello".to_string(), "World".to_string()];
    
    let v1: Vec<Message> = v.into_iter().map(Message::Write).collect();
}
