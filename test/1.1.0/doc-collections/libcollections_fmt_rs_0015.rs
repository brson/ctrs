fn main() {
    // Hello {arg 0 (x)} is {arg 1 (0.01} with precision specified inline (5)}
    println!("Hello {0} is {1:.5}", "x", 0.01);
    
    // Hello {arg 1 (x)} is {arg 2 (0.01} with precision specified in arg 0 (5)}
    println!("Hello {1} is {2:.0$}", 5, "x", 0.01);
    
    // Hello {arg 0 (x)} is {arg 2 (0.01} with precision specified in arg 1 (5)}
    println!("Hello {0} is {2:.1$}", "x", 5, 0.01);
    
    // Hello {next arg (x)} is {second of next two args (0.01} with precision
    //                          specified in first of next two args (5)}
    println!("Hello {} is {:.*}",    "x", 5, 0.01);
    
    // Hello {next arg (x)} is {arg 2 (0.01} with precision
    //                          specified in its predecessor (5)}
    println!("Hello {} is {2:.*}",   "x", 5, 0.01);
}
