pub struct NegativeFilter;

impl super::FilterMatrix for NegativeFilter {
    fn get3x3(&self) -> Vec<Vec<f32>> {
        return vec![
            vec![0.0, 0.0, 0.0],
            vec![0.0, -1.0, 0.0],
            vec![0.0, 0.0, 0.0],
        ]
    }
}