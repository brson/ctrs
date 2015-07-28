fn get_log_state() -> i32 { 3 }

macro_rules! log {

    ($msg:expr) => {{

        let state: i32 = get_log_state();

        if state > 0 {

            println!("log({}): {}", state, $msg);

        }

    }};

}



fn main() {

    let state: &str = "reticulating splines";

    log!(state);

}

