fn main() {
    let captured_var = 10i;
    
    let closure_no_args = || println!("captured_var={}", captured_var);
    
    let closure_args = |arg: int| -> int {
      println!("captured_var={}, arg={}", captured_var, arg);
      arg // Note lack of semicolon after 'arg'
    };
    
    fn call_closure(c1: ||, c2: |int| -> int) {
      c1();
      c2(2);
    }
    
    call_closure(closure_no_args, closure_args);
    
}