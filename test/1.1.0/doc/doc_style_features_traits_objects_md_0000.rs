fn main() {
    struct Frame  { ... }
    struct Button { ... }
    struct Label  { ... }
    
    trait Widget  { ... }
    
    impl Widget for Frame  { ... }
    impl Widget for Button { ... }
    impl Widget for Label  { ... }
    
    impl Frame {
        fn new(contents: &[Box<Widget>]) -> Frame {
            ...
        }
    }
    
    fn make_gui() -> Box<Widget> {
        let b: Box<Widget> = box Button::new(...);
        let l: Box<Widget> = box Label::new(...);
    
        box Frame::new([b, l]) as Box<Widget>
    }
}
