fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` yields `&i32`. `into_iter()` yields `i32`.
    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    // A reference to what is yielded is `&&i32`. Destructure to `i32`.
    println!("Find 2 in vec1: {:?}", iter     .find(|&&x| x == 2));
    // A reference to what is yielded is `&i32`. Destructure to `i32`.
    println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` is normal. `into_iter()` for arrays unusually
    // yields `&i32`. These would not normally both be `&&`.
    println!("Find 2 in array1: {:?}", array1.iter()     .find(|&&x| x == 2));
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&&x| x == 2));
}

