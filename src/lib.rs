pub mod math;
pub struct Weight(f64);

pub struct Neuron {
    pub weight: Weight,
}

#[derive(Debug, Clone)]
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
    pub fn transpose(&mut self) {
        let mut output = self.array.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                output[y + x * self.height] = self.array[x + y * self.width];
            }
        }
        self.array = output;
    }
    pub fn dot(a: Matrix, b: Matrix) -> Matrix {
        let out = Self::new(vec![], a.width, a.height);

        assert_eq!(a.width, b.height);
        assert_eq!(a.height, b.width);

        for x in 0..usize::max(a.width, b.width) {
            for y in 0..usize::max(a.height, b.height) {
                // fuck idk
            }
        }

        out
    }
}
