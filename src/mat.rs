use std::ops::{Add, Div, Index, Mul, Sub};
use std::slice::SliceIndex;

use crate::vec::{Vec2, Vec3, Vec4};

macro_rules! mat2 {
    (
        $m00:expr, $m01:expr; 
        $m10:expr, $m11:expr
    ) => {
        Mat2(Vec2([
            Vec2([$m00, $m01]), 
            Vec2([$m10, $m11])
        ]))
    };
}

macro_rules! mat3 {
    (
        $m00:expr, $m01:expr, $m02:expr; 
        $m10:expr, $m11:expr, $m12:expr; 
        $m20:expr, $m21:expr, $m22:expr
    ) => {
        Mat3(Vec3([
            Vec3([$m00, $m01, $m02]),
            Vec3([$m10, $m11, $m12]),
            Vec3([$m20, $m21, $m22]),
        ]))
    };
}

macro_rules! mat4 {
    (
        $m00:expr, $m01:expr, $m02:expr, $m03:expr;
        $m10:expr, $m11:expr, $m12:expr, $m13:expr; 
        $m20:expr, $m21:expr, $m22:expr, $m23:expr;
        $m30:expr, $m31:expr, $m32:expr, $m33:expr
    ) => {
        Mat4(Vec4([
            Vec4([$m00, $m01, $m02, $m03]),
            Vec4([$m10, $m11, $m12, $m13]),
            Vec4([$m20, $m21, $m22, $m23]),
            Vec4([$m30, $m31, $m32, $m33]),
        ]))
    };
}

#[derive(Default, Clone)]
pub struct Mat2<T>(Vec2<Vec2<T>>);

impl<T, I: SliceIndex<[Vec2<T>]>> Index<I> for Mat2<T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        Index::index(&self.0, index)
    }
}

impl<T: Add<Output = T> + Copy, R: Into<T> + Copy> Add<R> for Mat2<T> {
    type Output = Self;

    fn add(self, rhs: R) -> Self::Output {
        mat2![
            self[0][0] + rhs.into(), self[0][1] + rhs.into();
            self[1][0] + rhs.into(), self[1][1] + rhs.into()
        ]
    }
}

impl<T: Add<Output = T> + Copy> Add<Mat2<T>> for Mat2<T> {
    type Output = Self;

    fn add(self, rhs: Mat2<T>) -> Self::Output {
        mat2![
            self[0][0] + rhs[0][0], self[0][1] + rhs[0][1];
            self[1][0] + rhs[1][0], self[1][1] + rhs[1][1]
        ]
    }
}

impl<T: Sub<Output = T> + Copy, R: Into<T> + Copy> Sub<R> for Mat2<T> {
    type Output = Self;

    fn sub(self, rhs: R) -> Self::Output {
        mat2![
            self[0][0] - rhs.into(), self[0][1] - rhs.into();
            self[1][0] - rhs.into(), self[1][1] - rhs.into()
        ]
    }
}

impl<T: Sub<Output = T> + Copy> Sub<Mat2<T>> for Mat2<T> {
    type Output = Self;

    fn sub(self, rhs: Mat2<T>) -> Self::Output {
        mat2![
            self[0][0] - rhs[0][0], self[0][1] - rhs[0][1];
            self[1][0] - rhs[1][0], self[1][1] - rhs[1][1]
        ]
    }
}

impl<T: Mul<Output = T> + Copy, R: Into<T> + Copy> Mul<R> for Mat2<T> {
    type Output = Self;

    fn mul(self, rhs: R) -> Self::Output {
        mat2![
            self[0][0] * rhs.into(), self[0][1] * rhs.into();
            self[1][0] * rhs.into(), self[1][1] * rhs.into()
        ]
    }
}

impl<T: Mul<Output = T> + Copy> Mul<Mat2<T>> for Mat2<T> {
    type Output = Self;

    fn mul(self, rhs: Mat2<T>) -> Self::Output {
        mat2![
            self[0][0] * rhs[0][0], self[0][1] * rhs[0][1];
            self[1][0] * rhs[1][0], self[1][1] * rhs[1][1]
        ]
    }
}

impl<T: Div<Output = T> + Copy, R: Into<T> + Copy> Div<R> for Mat2<T> {
    type Output = Self;

    fn div(self, rhs: R) -> Self::Output {
        mat2![
            self[0][0] / rhs.into(), self[0][1] / rhs.into();
            self[1][0] / rhs.into(), self[1][1] / rhs.into()
        ]
    }
}

impl<T: Div<Output = T> + Copy> Div<Mat2<T>> for Mat2<T> {
    type Output = Self;

    fn div(self, rhs: Mat2<T>) -> Self::Output {
        mat2![
            self[0][0] / rhs[0][0], self[0][1] / rhs[0][1];
            self[1][0] / rhs[1][0], self[1][1] / rhs[1][1]
        ]
    }
}

#[derive(Default, Clone)]
pub struct Mat3<T>(Vec3<Vec3<T>>);

impl<T, I: SliceIndex<[Vec3<T>]>> Index<I> for Mat3<T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        Index::index(&self.0, index)
    }
}

impl<T: Add<Output = T> + Copy, R: Into<T> + Copy> Add<R> for Mat3<T> {
    type Output = Self;

    fn add(self, rhs: R) -> Self::Output {
        mat3![
            self[0][0] + rhs.into(), self[0][1] + rhs.into(), self[0][2] + rhs.into();
            self[1][0] + rhs.into(), self[1][1] + rhs.into(), self[1][2] + rhs.into();
            self[2][0] + rhs.into(), self[2][1] + rhs.into(), self[2][2] + rhs.into()
        ]
    }
}

impl<T: Add<Output = T> + Copy> Add<Mat2<T>> for Mat3<T> {
    type Output = Self;

    fn add(self, rhs: Mat2<T>) -> Self::Output {
        mat3![
            self[0][0] + rhs[0][0], self[0][1] + rhs[0][1], self[0][2] + rhs[0][2];
            self[1][0] + rhs[1][0], self[1][1] + rhs[1][1], self[1][2] + rhs[1][2];
            self[2][0] + rhs[2][0], self[2][1] + rhs[2][1], self[2][2] + rhs[2][2]
        ]
    }
}

impl<T: Sub<Output = T> + Copy, R: Into<T> + Copy> Sub<R> for Mat3<T> {
    type Output = Self;

    fn sub(self, rhs: R) -> Self::Output {
        mat3![
            self[0][0] - rhs.into(), self[0][1] - rhs.into(), self[0][2] - rhs.into();
            self[1][0] - rhs.into(), self[1][1] - rhs.into(), self[1][2] - rhs.into();
            self[2][0] - rhs.into(), self[2][1] - rhs.into(), self[2][2] - rhs.into()
        ]
    }
}

impl<T: Sub<Output = T> + Copy> Sub<Mat2<T>> for Mat3<T> {
    type Output = Self;

    fn sub(self, rhs: Mat2<T>) -> Self::Output {
        mat3![
            self[0][0] - rhs[0][0], self[0][1] - rhs[0][1], self[0][2] - rhs[0][2];
            self[1][0] - rhs[1][0], self[1][1] - rhs[1][1], self[1][2] - rhs[1][2];
            self[2][0] - rhs[2][0], self[2][1] - rhs[2][1], self[2][2] - rhs[2][2]
        ]
    }
}

impl<T: Mul<Output = T> + Copy, R: Into<T> + Copy> Mul<R> for Mat3<T> {
    type Output = Self;

    fn mul(self, rhs: R) -> Self::Output {
        mat3![
            self[0][0] * rhs.into(), self[0][1] * rhs.into(), self[0][2] * rhs.into();
            self[1][0] * rhs.into(), self[1][1] * rhs.into(), self[1][2] * rhs.into();
            self[2][0] * rhs.into(), self[2][1] * rhs.into(), self[2][2] * rhs.into()
        ]
    }
}

impl<T: Mul<Output = T> + Copy> Mul<Mat2<T>> for Mat3<T> {
    type Output = Self;

    fn mul(self, rhs: Mat2<T>) -> Self::Output {
        mat3![
            self[0][0] * rhs[0][0], self[0][1] * rhs[0][1], self[0][2] * rhs[0][2];
            self[1][0] * rhs[1][0], self[1][1] * rhs[1][1], self[1][2] * rhs[1][2];
            self[2][0] * rhs[2][0], self[2][1] * rhs[2][1], self[2][2] * rhs[2][2]
        ]
    }
}

impl<T: Div<Output = T> + Copy, R: Into<T> + Copy> Div<R> for Mat3<T> {
    type Output = Self;

    fn div(self, rhs: R) -> Self::Output {
        mat3![
            self[0][0] / rhs.into(), self[0][1] / rhs.into(), self[0][2] / rhs.into();
            self[1][0] / rhs.into(), self[1][1] / rhs.into(), self[1][2] / rhs.into();
            self[2][0] / rhs.into(), self[2][1] / rhs.into(), self[2][2] / rhs.into()
        ]
    }
}

impl<T: Div<Output = T> + Copy> Div<Mat2<T>> for Mat3<T> {
    type Output = Self;

    fn div(self, rhs: Mat2<T>) -> Self::Output {
        mat3![
            self[0][0] / rhs[0][0], self[0][1] / rhs[0][1], self[0][2] / rhs[0][2];
            self[1][0] / rhs[1][0], self[1][1] / rhs[1][1], self[1][2] / rhs[1][2];
            self[2][0] / rhs[2][0], self[2][1] / rhs[2][1], self[2][2] / rhs[2][2]
        ]
    }
}

#[derive(Default, Clone)]
pub struct Mat4<T>(Vec4<Vec4<T>>);

impl<T, I: SliceIndex<[Vec4<T>]>> Index<I> for Mat4<T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        Index::index(&self.0, index)
    }
}

impl<T: Add<Output = T> + Copy, R: Into<T> + Copy> Add<R> for Mat4<T> {
    type Output = Self;

    fn add(self, rhs: R) -> Self::Output {
        mat4![
            self[0][0] + rhs.into(), self[0][1] + rhs.into(), self[0][2] + rhs.into(), self[0][3] + rhs.into();
            self[1][0] + rhs.into(), self[1][1] + rhs.into(), self[1][2] + rhs.into(), self[1][3] + rhs.into();
            self[2][0] + rhs.into(), self[2][1] + rhs.into(), self[2][2] + rhs.into(), self[2][3] + rhs.into();
            self[3][0] + rhs.into(), self[3][1] + rhs.into(), self[3][2] + rhs.into(), self[3][3] + rhs.into()
        ]
    }
}

impl<T: Add<Output = T> + Copy> Add<Mat2<T>> for Mat4<T> {
    type Output = Self;

    fn add(self, rhs: Mat2<T>) -> Self::Output {
        mat4![
            self[0][0] + rhs[0][0], self[0][1] + rhs[0][1], self[0][2] + rhs[0][2], self[0][3] + rhs[0][3];
            self[1][0] + rhs[1][0], self[1][1] + rhs[1][1], self[1][2] + rhs[1][2], self[1][3] + rhs[1][3];
            self[2][0] + rhs[2][0], self[2][1] + rhs[2][1], self[2][2] + rhs[2][2], self[2][3] + rhs[2][3];
            self[3][0] + rhs[3][0], self[3][1] + rhs[3][1], self[3][2] + rhs[3][2], self[3][3] + rhs[3][3]
        ]
    }
}

impl<T: Sub<Output = T> + Copy, R: Into<T> + Copy> Sub<R> for Mat4<T> {
    type Output = Self;

    fn sub(self, rhs: R) -> Self::Output {
        mat4![
            self[0][0] - rhs.into(), self[0][1] - rhs.into(), self[0][2] - rhs.into(), self[0][3] - rhs.into();
            self[1][0] - rhs.into(), self[1][1] - rhs.into(), self[1][2] - rhs.into(), self[1][3] - rhs.into();
            self[2][0] - rhs.into(), self[2][1] - rhs.into(), self[2][2] - rhs.into(), self[2][3] - rhs.into();
            self[3][0] - rhs.into(), self[3][1] - rhs.into(), self[3][2] - rhs.into(), self[3][3] - rhs.into()
        ]
    }
}

impl<T: Sub<Output = T> + Copy> Sub<Mat2<T>> for Mat4<T> {
    type Output = Self;

    fn sub(self, rhs: Mat2<T>) -> Self::Output {
        mat4![
            self[0][0] - rhs[0][0], self[0][1] - rhs[0][1], self[0][2] - rhs[0][2], self[0][3] - rhs[0][3];
            self[1][0] - rhs[1][0], self[1][1] - rhs[1][1], self[1][2] - rhs[1][2], self[1][3] - rhs[1][3];
            self[2][0] - rhs[2][0], self[2][1] - rhs[2][1], self[2][2] - rhs[2][2], self[2][3] - rhs[2][3];
            self[3][0] - rhs[3][0], self[3][1] - rhs[3][1], self[3][2] - rhs[3][2], self[3][3] - rhs[3][3]
        ]
    }
}

impl<T: Mul<Output = T> + Copy, R: Into<T> + Copy> Mul<R> for Mat4<T> {
    type Output = Self;

    fn mul(self, rhs: R) -> Self::Output {
        mat4![
            self[0][0] * rhs.into(), self[0][1] * rhs.into(), self[0][2] * rhs.into(), self[0][3] * rhs.into();
            self[1][0] * rhs.into(), self[1][1] * rhs.into(), self[1][2] * rhs.into(), self[1][3] * rhs.into();
            self[2][0] * rhs.into(), self[2][1] * rhs.into(), self[2][2] * rhs.into(), self[2][3] * rhs.into();
            self[3][0] * rhs.into(), self[3][1] * rhs.into(), self[3][2] * rhs.into(), self[3][3] * rhs.into()
        ]
    }
}

impl<T: Mul<Output = T> + Copy> Mul<Mat2<T>> for Mat4<T> {
    type Output = Self;

    fn mul(self, rhs: Mat2<T>) -> Self::Output {
        mat4![
            self[0][0] * rhs[0][0], self[0][1] * rhs[0][1], self[0][2] * rhs[0][2], self[0][3] * rhs[0][3];
            self[1][0] * rhs[1][0], self[1][1] * rhs[1][1], self[1][2] * rhs[1][2], self[1][3] * rhs[1][3];
            self[2][0] * rhs[2][0], self[2][1] * rhs[2][1], self[2][2] * rhs[2][2], self[2][3] * rhs[2][3];
            self[3][0] * rhs[3][0], self[3][1] * rhs[3][1], self[3][2] * rhs[3][2], self[3][3] * rhs[3][3]
        ]
    }
}

impl<T: Div<Output = T> + Copy, R: Into<T> + Copy> Div<R> for Mat4<T> {
    type Output = Self;

    fn div(self, rhs: R) -> Self::Output {
        mat4![
            self[0][0] / rhs.into(), self[0][1] / rhs.into(), self[0][2] / rhs.into(), self[0][3] / rhs.into();
            self[1][0] / rhs.into(), self[1][1] / rhs.into(), self[1][2] / rhs.into(), self[1][3] / rhs.into();
            self[2][0] / rhs.into(), self[2][1] / rhs.into(), self[2][2] / rhs.into(), self[2][3] / rhs.into();
            self[3][0] / rhs.into(), self[3][1] / rhs.into(), self[3][2] / rhs.into(), self[3][3] / rhs.into()
        ]
    }
}

impl<T: Div<Output = T> + Copy> Div<Mat2<T>> for Mat4<T> {
    type Output = Self;

    fn div(self, rhs: Mat2<T>) -> Self::Output {
        mat4![
            self[0][0] / rhs[0][0], self[0][1] / rhs[0][1], self[0][2] / rhs[0][2], self[0][3] / rhs[0][3];
            self[1][0] / rhs[1][0], self[1][1] / rhs[1][1], self[1][2] / rhs[1][2], self[1][3] / rhs[1][3];
            self[2][0] / rhs[2][0], self[2][1] / rhs[2][1], self[2][2] / rhs[2][2], self[2][3] / rhs[2][3];
            self[3][0] / rhs[3][0], self[3][1] / rhs[3][1], self[3][2] / rhs[3][2], self[3][3] / rhs[3][3]
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mat_default() {
        let m: Mat2<f64> = Mat2::default();
        assert_eq!(m[0][0] == 0.0, true)
    }
}
