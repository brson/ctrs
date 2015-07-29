fn main() {
    let x: Vec<u32> = {

        let mut temp_vec = Vec::new();

        temp_vec.push(1);

        temp_vec.push(2);

        temp_vec.push(3);

        temp_vec

    };

    assert_eq!(x, [1, 2, 3]);

}
