use serde::{Deserialize, Deserializer};
use std::ops::Mul;
use vector::Vector3;

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Matrix33 {
    pub elements: [[f64; 3]; 3],
}

impl Matrix33 {
    pub fn from_vecs(x: &Vector3, y: &Vector3, z: &Vector3) -> Matrix33 {
        Matrix33 {
            elements: [[x.x, y.x, z.x], [x.y, y.y, z.y], [x.z, y.z, z.z]],
        }
    }

    pub fn identity() -> Matrix33 {
        Matrix33 {
            elements: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    pub fn transpose(&self) -> Matrix33 {
        Matrix33 {
            elements: [
                [
                    self.elements[0][0],
                    self.elements[1][0],
                    self.elements[2][0],
                ],
                [
                    self.elements[0][1],
                    self.elements[1][1],
                    self.elements[2][1],
                ],
                [
                    self.elements[0][2],
                    self.elements[1][2],
                    self.elements[2][2],
                ],
            ],
        }
    }
}

impl Mul<Vector3> for Matrix33 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.elements[0][0] * other.x
                + self.elements[0][1] * other.y
                + self.elements[0][2] * other.z,
            y: self.elements[1][0] * other.x
                + self.elements[1][1] * other.y
                + self.elements[1][2] * other.z,
            z: self.elements[2][0] * other.x
                + self.elements[2][1] * other.y
                + self.elements[2][2] * other.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_identity() {
        assert_eq!(
            Matrix33::identity(),
            Matrix33 {
                elements: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
            }
        );
    }

    #[test]
    fn test_matrix_transpose_of_identity_is_idendtity() {
        assert_eq!(Matrix33::identity().transpose(), Matrix33::identity());
    }

    #[test]
    fn test_matrix_transpose() {
        let mut matrix = Matrix33::identity();
        matrix.elements[1][0] = 1.0;
        let mut result_matrix = Matrix33::identity();
        result_matrix.elements[0][1] = 1.0;
        assert_eq!(matrix.transpose(), result_matrix);
    }
}
