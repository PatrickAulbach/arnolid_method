use crate::common::matrix::matrix::{Vector, Matrix};
use num::Num;
use std::ops::{Mul, Add, AddAssign};
use std::marker::PhantomData;
use std::str::FromStr;
use crate::common::matrix::square_root::Sqrt;

pub struct MatrixOperations<T> {
    phantom_data: PhantomData<T>
}

impl<T> MatrixOperations<T> where T: Mul<Output = T> + Add<Output = T>, T: FromStr + Copy + Num + AddAssign + Sqrt {
    //compute alpha * A + beta * B
    pub fn add(a: Matrix<T>, b: Matrix<T>, alpha: T, beta: T) -> Matrix<T> {
        if alpha == T::zero() && beta == T::one() {
            b
        } else if beta == T::zero() && alpha == T::one() {
            a
        } else {
            let mut data: Vec<Vec<T>> = Vec::new();
            for i in 0..a.ncols() {
                let mut column_vec: Vec<T> = Vec::new();
                for j in 0..a.nrows() {
                    column_vec.push(a.data()[i][j] * alpha + b.data()[i][j] * beta);
                }
                data.push(column_vec);
            }
            Matrix::new(data)
        }
    }

    pub fn euclidean_norm(vector: Vector<T>) -> T {
        let mut norm = T::zero();

        for i in 0..vector.nrows() {
            norm += vector.data()[0][i] * vector.data()[0][i];
        }

        norm.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;


    #[test]
    fn add_with_alpha_zero_should_return_b() {
        let first_matrix: Matrix<f64> = Matrix::new(
            vec![
                vec![1.0, 1.0, 1.0],
                vec![1.0, 1.0, 1.0],
                vec![1.0, 1.0, 1.0]
            ]
        );
        let second_matrix: Matrix<f64> = Matrix::new(
            vec![
                vec![1.0, 1.0, 1.0],
                vec![1.0, 1.0, 1.0],
                vec![1.0, 1.0, 1.0]
            ]
        );

        let added_matrix: Matrix<f64> = MatrixOperations::add(first_matrix, second_matrix, 0f64, 1f64);

        assert_eq!(added_matrix.data()[0], vec![1.0, 1.0, 1.0]);
        assert_eq!(added_matrix.data()[1], vec![1.0, 1.0, 1.0]);
        assert_eq!(added_matrix.data()[2], vec![1.0, 1.0, 1.0]);
    }

    #[test]
    fn add_with_alpha_zero_and_beta_greater_one_should_return_beta_times_b() {
        let first_matrix: Matrix<f64> = Matrix::new(
            vec![
                vec![1.0, 1.0, 1.0],
                vec![1.0, 1.0, 1.0],
                vec![1.0, 1.0, 1.0]
            ]
        );
        let second_matrix: Matrix<f64> = Matrix::new(
            vec![
                vec![1.0, 2.0, 3.0],
                vec![4.0, 5.0, 6.0],
                vec![7.0, 8.0, 9.0]
            ]
        );

        let added_matrix: Matrix<f64> = MatrixOperations::add(first_matrix, second_matrix, 0f64, 2f64);

        assert_eq!(added_matrix.data()[0], vec![2.0, 4.0, 6.0]);
        assert_eq!(added_matrix.data()[1], vec![8.0, 10.0, 12.0]);
        assert_eq!(added_matrix.data()[2], vec![14.0, 16.0, 18.0]);
    }

    #[test]
    fn euclidian_norm_should_be_correct() {
        let vector: Vector<f64> = Vector::new(
            vec![
                vec![1.0, 2.0, 3.0]
            ]
        );

        let norm = MatrixOperations::euclidean_norm(vector);

        assert_approx_eq!(3.74165738677, norm, 1e-3f64);
    }

}

/*
pub(crate) mod vector_operations {




    pub fn gram_schmidt(base_vector_matrix: &mut Vec<Vec<f64>>, hessenberg_matrix: &mut Vec<Vec<f64>>, iteration_count: usize) {
        for i in 0..iteration_count {
            //hj,k-1 = qj*qk
            hessenberg_matrix[i][iteration_count - 1] = scalar_product(&base_vector_matrix[i], &base_vector_matrix[iteration_count]);
            //qk = qk - hj,k-1*qj
            let second_vector = scalar_vector_multiplication(hessenberg_matrix[i][iteration_count - 1], &mut base_vector_matrix[i]);

            vector_addition_subtraction(&mut base_vector_matrix[iteration_count],
                                        &second_vector,
                                        true);
        }
    }


}

 */