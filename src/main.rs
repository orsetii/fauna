use fauna::matrix::Matrix;

fn main() {

    let inputs = vec![1.0, 2.0, 3.0, 2.5, 2.0, 5.0, -1.0, 2.0, -1.5, 2.7, 3.3, -0.8];
    let weights = vec![0.2, 0.8, -0.5, 1.0, 0.5, -0.91, 0.26, -0.5, -0.26, -0.27, 0.17, 0.87];

    let inputs_matrix = Matrix::new(inputs, 4, 3);
    let mut weights_matrix = Matrix::new(weights, 4, 3);

    println!("Before transpose: {:#?}", weights_matrix);
    weights_matrix.transpose();
    println!("After transpose: {:#?}", weights_matrix);

    let mut result = Matrix::dot(&inputs_matrix, &weights_matrix);
    println!("Before bias: {:#?}", result);
    result.add_bias(&[2.0, 3.0, 0.5]);

    println!("After bias: {:#?}", result);
}
