pub mod product {

    pub fn dot(a1: Vec<f64>, a2: Vec<f64>) -> f64 {
        assert_eq!(a1.len(), a2.len(), "Vectors must have the same length");
        a1.iter()
            .zip(a2.iter())
            .fold(0.0, |acc, (&x, &y)| acc + x * y)
    }

    pub fn dot_layers(a1: Vec<Vec<f64>>, a2: Vec<f64>) -> Vec<f64> {
        assert_eq!(
            a1.iter()
                .map(|v| v.len())
                .collect::<std::collections::HashSet<_>>()
                .len(),
            1,
            "All sub-arrays in a1 must have the same length"
        );
        assert_eq!(
            a1.get(0).map(|v| v.len()).unwrap_or(0),
            a2.len(),
            "The length of a2 must match the length of sub-arrays in a1"
        );

        a1.iter()
            .map(|sub_arr| {
                sub_arr
                    .iter()
                    .zip(a2.iter())
                    .fold(0.0, |acc, (&x, &y)| acc + x * y)
            })
            .collect()
    }
}
