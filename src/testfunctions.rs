// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! TODO Documentation

use std::iter::Sum;
use ndarray::{Array1, Array2};
use num::{Float, FromPrimitive};

/// Rosenbrock test function
///
/// Parameters are usually: `a = 1` and `b = 100`
/// TODO: make this multidimensional
pub fn rosenbrock<T: Float + FromPrimitive>(param: &[T], a: T, b: T) -> T {
    let num2 = T::from_f64(2.0).unwrap();
    (a - param[0]).powf(num2) + b * (param[1] - param[0].powf(num2)).powf(num2)
}

/// Derivative of 2D Rosenbrock function
pub fn rosenbrock_derivative<T: Float + FromPrimitive>(param: &[T], a: T, b: T) -> Vec<T> {
    let num2 = T::from_f64(2.0).unwrap();
    let num3 = T::from_f64(3.0).unwrap();
    let num4 = T::from_f64(4.0).unwrap();
    let x = param[0];
    let y = param[1];
    let mut out = vec![];
    out.push(-num2 * a + num4 * b * x.powf(num3) - num4 * b * x * y + num2 * x);
    out.push(num2 * b * (y - x.powf(num2)));
    out
}

/// Hessian of 2D Rosenbrock function
pub fn rosenbrock_hessian<T: Float + FromPrimitive>(param: &[T], _a: T, b: T) -> Vec<T> {
    let num2 = T::from_f64(2.0).unwrap();
    let num4 = T::from_f64(4.0).unwrap();
    let num12 = T::from_f64(12.0).unwrap();
    let x = param[0];
    let y = param[1];
    let mut out = vec![];
    // d/dxdx
    out.push(num12 * b * x.powf(num2) - num4 * b * y + num2);
    // d/dxdy
    out.push(-num4 * b * x);
    // d/dydx
    out.push(-num4 * b * x);
    // d/dydy
    out.push(num2 * b);
    out
}

/// Rosenbrock test function, taking ndarray
///
/// Parameters are usually: `a = 1` and `b = 100`
/// TODO: make this multidimensional
pub fn rosenbrock_nd<T: Float + FromPrimitive>(param: &Array1<T>, a: T, b: T) -> T {
    let num2 = T::from_f64(2.0).unwrap();
    (a - param[0]).powf(num2) + b * (param[1] - param[0].powf(num2)).powf(num2)
}

/// Derivative of 2D Rosenbrock function, returning an ndarray
pub fn rosenbrock_derivative_nd<T: Float + FromPrimitive>(
    param: &Array1<T>,
    a: T,
    b: T,
) -> Array1<T> {
    let num2 = T::from_f64(2.0).unwrap();
    let num3 = T::from_f64(3.0).unwrap();
    let num4 = T::from_f64(4.0).unwrap();
    let x = param[0];
    let y = param[1];
    let mut out = Array1::zeros(2);
    out[[0]] = -num2 * a + num4 * b * x.powf(num3) - num4 * b * x * y + num2 * x;
    out[[1]] = num2 * b * (y - x.powf(num2));
    out
}

/// Hessian of 2D Rosenbrock function, returning an ndarray
pub fn rosenbrock_hessian_nd<T: Float + FromPrimitive>(
    param: &Array1<T>,
    _a: T,
    b: T,
) -> Array2<T> {
    let num2 = T::from_f64(2.0).unwrap();
    let num4 = T::from_f64(4.0).unwrap();
    let num12 = T::from_f64(12.0).unwrap();
    let x = param[0];
    let y = param[1];
    let mut out = Array2::zeros((2, 2));
    // d/dxdx
    out[[0, 0]] = num12 * b * x.powf(num2) - num4 * b * y + num2;
    // d/dxdy
    out[[0, 1]] = -num4 * b * x;
    // d/dydx
    out[[1, 0]] = -num4 * b * x;
    // d/dydy
    out[[1, 1]] = num2 * b;
    out
}

/// Sphere test function
pub fn sphere<T: Float + FromPrimitive + Sum>(param: &[T]) -> T {
    let num2 = T::from_f64(2.0).unwrap();
    param.iter().map(|x| x.powf(num2)).sum()
}

/// Derivative of sphere test function
pub fn sphere_derivative<T: Float + FromPrimitive>(param: &[T]) -> Vec<T> {
    let num2 = T::from_f64(2.0).unwrap();
    param.iter().map(|x| num2 * *x).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr1;

    #[test]
    fn compare_rosenbrock_funcs() {
        assert_eq!(
            rosenbrock(&vec![1.0_f32, 1.0_f32], 1.0, 100.0),
            rosenbrock_nd(&arr1(&[1.0, 1.0]), 1.0, 100.0)
        );
    }

    #[test]
    fn rosenbrock_optimum_f32() {
        assert_eq!(rosenbrock(&vec![1.0_f32, 1.0_f32], 1.0, 100.0), 0.0);
    }

    #[test]
    fn rosenbrock_optimum_f64() {
        assert_eq!(rosenbrock(&vec![1.0, 1.0], 1.0, 100.0), 0.0);
    }

    #[test]
    fn rosenbrock_nd_optimum_f64() {
        assert_eq!(rosenbrock_nd(&arr1(&[1.0, 1.0]), 1.0, 100.0), 0.0);
    }

    #[test]
    fn rosenbrock_nd_optimum_f32() {
        assert_eq!(
            rosenbrock_nd(&arr1(&[1.0_f32, 1.0_f32]), 1.0_f32, 100.0_f32),
            0.0_f32
        );
    }

}
