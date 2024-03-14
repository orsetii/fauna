use fauna::Matrix;

fn main() {
    let inputs = [
        [1.0, 2.0, 3.0, 2.5].to_vec(),
        [2.0, 5.0, -1.0, 2.0].to_vec(),
        [-1.5, 2.7, 3.3, -0.8].to_vec(),
    ]
    .to_vec();
    let weights = [
        [0.2, 0.8, -0.5, 1.0].to_vec(),
        [0.5, -0.91, 0.26, -0.5].to_vec(),
        [-0.26, -0.27, 0.17, 0.87].to_vec(),
    ]
    .to_vec();
    let biases = vec![2.0, 3.0, 0.5];

    #[rustfmt::skip]
    let mut m = Matrix::new(
        vec![
            0.0, 1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0, 9.0,
            10.0, 11.0, 12.0, 13.0, 14.0,
        ], 
        5, 
        3
    );
    println!("{:?}", m);
    m.transpose();
    println!("{:?}", m);

    let mut m2 = Matrix::new(vec![2.0, 3.0, 4.0], 3, 1);
    println!("{:?}", m2);
    m2.transpose();
    println!("{:?}", m2);
}
