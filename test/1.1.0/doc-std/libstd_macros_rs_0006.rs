fn main() {
    let key: Option<&'static str> = option_env!("SECRET_KEY");    println!("the secret key might be: {:?}", key);}
