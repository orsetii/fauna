
use std::fmt;

pub struct Matrix {
    array: Vec<f64>,
    pub width: usize,
    pub height: usize,
}

impl Matrix {
    pub fn new(array: Vec<f64>, width: usize, height: usize) -> Self {
        Self {
            array,
            width,
            height,
        }
    }

    /// Matrix transposition
    pub fn transpose(&mut self) {
        let mut output = vec![0.00; self.width * self.height];

        for x in 0..self.width {
            for y in 0..self.height {
                               output[y + x * self.height] = self.array[x + y * self.width];
            }
        }

        self.array = output;
        std::mem::swap(&mut self.width, &mut self.height);
    }


    /// Matrix dot product (AKA Matrix multiplication or Matrix Product)
    pub fn dot(a: &Matrix, b: &Matrix) -> Matrix {
        if a.width != b.height {
            panic!("Matrices are not compatible for multiplication");
        }

        let mut result = Vec::with_capacity(a.height * b.width);

        for i in 0..a.height {
            for j in 0..b.width {
                let mut sum = 0.0;
                for k in 0..a.width {
                    sum += a.array[i * a.width + k] * b.array[k * b.width + j];
                }
                result.push(sum);
            }
        }

        Matrix::new(result, b.width, a.height)
    }

    /// Adds a 1-Dimensional array of bias values to the matrix,
    /// one per row in the matrix.
    /// `Bias.len()` **must** be equal to the width of the
    /// `Matrix`'s row(s)
    pub fn add_bias(&mut self, bias_list: &[f64]) {
        assert_eq!(bias_list.len(), self.width);
        for i in 0..self.height {
            for j in 0..self.width {
                self.array[j + i * self.height] += bias_list[j];
            }
        }
    }
}



impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        output.push_str("Matrix: \n");

        for y in 0..self.height {
            let row_start = y * self.width;
            let row_end = row_start + self.width;
            let row = &self.array[row_start..row_end];

            output.push_str(&row.iter().map(|x| format!("{: >6.2}", x)).collect::<Vec<_>>().join(" "));
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}

