use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Matrix([f64; 6]);
const IDENTITY_MATRIX: [f64; 6] = [1., 0., 0., 1., 0., 0.];

impl Matrix {
    pub fn new(inner: [f64; 6]) -> Self {
        Self(inner)
    }

    // Typical matrix multiplication is not possible with the underlying inner
    // matrix type (i.e. (3x2) x (3x2)) so we essentially add the identity vector
    // to the last column of each. This means that the first four operations have
    // 0 * self[0..4] as the final term, which evaluates to 0 and we can omit.
    // Similarly, the final terms of the last two operations will be 1 * self[4..6]
    // which is just self[4] and self[5] respectively, so we can remove the
    // multiplication
    pub fn concatenate(&mut self, other: &Self) {
        let [a, b, c, d, e, f] = other.0;
        let [prev_a, prev_b, prev_c, prev_d, prev_e, prev_f] = &self.0;
        self.0 = [
            // We can omit the final term for the first four entries since it
            // will be adding 0
            a * prev_a + b * prev_c,
            a * prev_b + b * prev_d,
            c * prev_a + d * prev_c,
            c * prev_b + d * prev_d,
            e * prev_a + f * prev_c + prev_e,
            e * prev_b + f * prev_d + prev_f,
        ];
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Matrix(IDENTITY_MATRIX)
    }
}

impl Deref for Matrix {
    type Target = [f64; 6];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
