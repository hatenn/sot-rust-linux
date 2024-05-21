use maths_rs::Vec3d;
use num_complex::Complex;
use crate::gametypes::FVector;

pub fn dot_p(v1: Vec3d, v2: Vec3d) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn normalize_vector(p: FVector) -> FVector {
    let w = f32::sqrt(p.x * p.x + p.y * p.y + p.z * p.z);
    FVector {
        x: p.x / w,
        y: p.y / w,
        z: p.z / w,
    }
}

fn complex_cbrt(z: Complex<f64>) -> Complex<f64> {
    z.powf(1.0 / 3.0)
}

fn complex_sqrt(z: Complex<f64>) -> Complex<f64> {
    z.sqrt()
}

pub fn solve_quartic(coefficients: &[Complex<f64>; 5], roots: &mut [Complex<f64>; 4]) {
    let a = coefficients[4];
    let b = coefficients[3] / a;
    let c = coefficients[2] / a;
    let d = coefficients[1] / a;
    let e = coefficients[0] / a;

    let q1 = c * c - 3.0 * b * d + 12.0 * e;
    let q2 = 2.0 * c * c * c - 9.0 * b * c * d + 27.0 * d * d + 27.0 * b * b * e - 72.0 * c * e;
    let q3 = 8.0 * b * c - 16.0 * d - 2.0 * b * b * b;
    let q4 = 3.0 * b * b - 8.0 * c;

    let q5 = complex_cbrt(q2 / 2.0 + complex_sqrt(q2 * q2 / 4.0 - q1 * q1 * q1));
    let q6 = (q1 / q5 + q5) / 3.0;
    let q7 = 2.0 * complex_sqrt(q4 / 12.0 + q6);

    roots[0] = (-b - q7 - complex_sqrt(4.0 * q4 / 6.0 - 4.0 * q6 - q3 / q7)) / 4.0;
    roots[1] = (-b - q7 + complex_sqrt(4.0 * q4 / 6.0 - 4.0 * q6 - q3 / q7)) / 4.0;
    roots[2] = (-b + q7 - complex_sqrt(4.0 * q4 / 6.0 - 4.0 * q6 + q3 / q7)) / 4.0;
    roots[3] = (-b + q7 + complex_sqrt(4.0 * q4 / 6.0 - 4.0 * q6 + q3 / q7)) / 4.0;
}