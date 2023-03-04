#![allow(dead_code)]

use std::cmp::{Eq, PartialEq};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use std::slice::SliceIndex;

#[macro_export]
macro_rules! vec2 {
    ($v0:expr, $v1:expr) => {
        Vec2([$v0, $v1])
    };
}

#[macro_export]
macro_rules! vec3 {
    ($v0:expr, $v1:expr, $v2:expr) => {
        Vec3([$v0, $v1, $v2])
    };
}

#[macro_export]
macro_rules! vec4 {
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr) => {
        Vec4([$v0, $v1, $v2, $v3])
    };
}

#[derive(Debug, Default, Clone)]
pub struct Vec2<T>(pub [T; 2]);

impl<T, I: SliceIndex<[T]>> Index<I> for Vec2<T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        Index::index(&self.0, index)
    }
}

impl<T, I: SliceIndex<[T]>> IndexMut<I> for Vec2<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut self.0, index)
    }
}

impl<T: PartialEq> PartialEq for Vec2<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self[0] == rhs[0] && self[1] == rhs[1]
    }
}

impl<T: Eq> Eq for Vec2<T> {}

impl<T: Add<Output = T> + Copy, R: Into<T> + Copy> Add<R> for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: R) -> Self::Output {
        vec2![self[0] + rhs.into(), self.0[1] + rhs.into()]
    }
}

impl<T: Add<Output = T> + Copy> Add<Vec2<T>> for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Vec2<T>) -> Self::Output {
        vec2![self[0] + rhs[0], self[1] + rhs[1]]
    }
}

impl<T: Sub<Output = T> + Copy, R: Into<T> + Copy> Sub<R> for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: R) -> Self::Output {
        vec2![self[0] - rhs.into(), self[1] - rhs.into()]
    }
}

impl<T: Sub<Output = T> + Copy> Sub<Vec2<T>> for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        vec2![self[0] - rhs[0], self[1] - rhs[1]]
    }
}

impl<T: Mul<Output = T> + Copy, R: Into<T> + Copy> Mul<R> for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: R) -> Self::Output {
        vec2![self[0] * rhs.into(), self[1] * rhs.into()]
    }
}

impl<T: Mul<Output = T> + Copy> Mul<Vec2<T>> for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        vec2![self[0] * rhs[0], self[1] * rhs[1]]
    }
}

impl<T: Div<Output = T> + Copy, R: Into<T> + Copy> Div<R> for Vec2<T> {
    type Output = Self;

    fn div(self, rhs: R) -> Self::Output {
        vec2![self[0] / rhs.into(), self[1] / rhs.into()]
    }
}

impl<T: Div<Output = T> + Copy> Div<Vec2<T>> for Vec2<T> {
    type Output = Self;

    fn div(self, rhs: Vec2<T>) -> Self::Output {
        vec2![self[0] / rhs[0], self[1] / rhs[1]]
    }
}

impl<T: AddAssign<T> + Copy, R: Into<T> + Copy> AddAssign<R> for Vec2<T> {
    fn add_assign(&mut self, rhs: R) {
        self[0] += rhs.into();
        self[1] += rhs.into();
    }
}

impl<T: AddAssign<T> + Copy> AddAssign<Vec2<T>> for Vec2<T> {
    fn add_assign(&mut self, rhs: Vec2<T>) {
        self[0] += rhs[0];
        self[1] += rhs[1];
    }
}

impl<T: SubAssign<T> + Copy, R: Into<T> + Copy> SubAssign<R> for Vec2<T> {
    fn sub_assign(&mut self, rhs: R) {
        self[0] -= rhs.into();
        self[1] -= rhs.into();
    }
}

impl<T: SubAssign<T> + Copy> SubAssign<Vec2<T>> for Vec2<T> {
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
    }
}

impl<T: MulAssign<T> + Copy, R: Into<T> + Copy> MulAssign<R> for Vec2<T> {
    fn mul_assign(&mut self, rhs: R) {
        self[0] *= rhs.into();
        self[1] *= rhs.into();
    }
}

impl<T: MulAssign<T> + Copy> MulAssign<Vec2<T>> for Vec2<T> {
    fn mul_assign(&mut self, rhs: Vec2<T>) {
        self[0] *= rhs[0];
        self[1] *= rhs[1];
    }
}

impl<T: DivAssign<T> + Copy, R: Into<T> + Copy> DivAssign<R> for Vec2<T> {
    fn div_assign(&mut self, rhs: R) {
        self[0] /= rhs.into();
        self[1] /= rhs.into();
    }
}

impl<T: DivAssign<T> + Copy> DivAssign<Vec2<T>> for Vec2<T> {
    fn div_assign(&mut self, rhs: Vec2<T>) {
        self[0] /= rhs[0];
        self[1] /= rhs[1];
    }
}

impl<T: Copy> Vec2<T> {
    fn all(v: T) -> Vec2<T> {
        vec2![v, v]
    }

    fn x(&self) -> T {
        self[0]
    }

    fn y(&self) -> T {
        self[1]
    }
}

impl<T: Copy + Add<T, Output = T> + Mul<T, Output = T>> Vec2<T> {
    fn dot(&self, rhs: &Vec2<T>) -> T {
        self[0] * rhs[0] + self[1] * rhs[1]
    }
}

impl<T: Default + Copy + Sub<T, Output = T> + Mul<T, Output = T>> Vec2<T> {
    fn cross(&self, rhs: &Vec2<T>) -> Vec3<T> {
        vec3![
            T::default(),
            T::default(),
            self[0] * rhs[1] - self[1] * rhs[0]
        ]
    }
}

impl Vec2<f64> {
    fn square_length(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1]
    }

    fn length(&self) -> f64 {
        self.square_length().sqrt()
    }

    fn normalize(&self) -> Vec2<f64> {
        let len = self.length();
        vec2![self[0] / len, self[1] / len]
    }

    fn rotate_left_90(&self) -> Vec2<f64> {
        vec2![-self[1], self[0]]
    }

    fn rotate_right_90(&self) -> Vec2<f64> {
        vec2![self[1], -self[0]]
    }
}

#[derive(Debug, Default, Clone)]
pub struct Vec3<T>(pub [T; 3]);

impl<T, I: SliceIndex<[T]>> Index<I> for Vec3<T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        Index::index(&self.0, index)
    }
}

impl<T, I: SliceIndex<[T]>> IndexMut<I> for Vec3<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut self.0, index)
    }
}

impl<T: PartialEq> PartialEq for Vec3<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self[0] == rhs[0] && self[1] == rhs[1] && self[2] == rhs[2]
    }
}

impl<T: Eq> Eq for Vec3<T> {}

impl<T: Add<Output = T> + Copy, R: Into<T> + Copy> Add<R> for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: R) -> Self::Output {
        vec3![
            self[0] + rhs.into(),
            self[1] + rhs.into(),
            self[2] + rhs.into()
        ]
    }
}

impl<T: Add<Output = T> + Copy> Add<Vec3<T>> for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        vec3![self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]]
    }
}

impl<T: Sub<Output = T> + Copy, R: Into<T> + Copy> Sub<R> for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: R) -> Self::Output {
        vec3![
            self[0] - rhs.into(),
            self[1] - rhs.into(),
            self[2] - rhs.into()
        ]
    }
}

impl<T: Sub<Output = T> + Copy> Sub<Vec3<T>> for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        vec3![self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]]
    }
}

impl<T: Mul<Output = T> + Copy, R: Into<T> + Copy> Mul<R> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: R) -> Self::Output {
        vec3![
            self[0] * rhs.into(),
            self[1] * rhs.into(),
            self[2] * rhs.into()
        ]
    }
}

impl<T: Mul<Output = T> + Copy> Mul<Vec3<T>> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        vec3![self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]]
    }
}

impl<T: Div<Output = T> + Copy, R: Into<T> + Copy> Div<R> for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: R) -> Self::Output {
        vec3![
            self[0] / rhs.into(),
            self[1] / rhs.into(),
            self[2] / rhs.into()
        ]
    }
}

impl<T: Div<Output = T> + Copy> Div<Vec3<T>> for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: Vec3<T>) -> Self::Output {
        vec3![self[0] / rhs[0], self[1] / rhs[1], self[2] / rhs[2]]
    }
}

impl<T: AddAssign<T> + Copy, R: Into<T> + Copy> AddAssign<R> for Vec3<T> {
    fn add_assign(&mut self, rhs: R) {
        self[0] += rhs.into();
        self[1] += rhs.into();
        self[2] += rhs.into();
    }
}

impl<T: AddAssign<T> + Copy> AddAssign<Vec3<T>> for Vec3<T> {
    fn add_assign(&mut self, rhs: Vec3<T>) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl<T: SubAssign<T> + Copy, R: Into<T> + Copy> SubAssign<R> for Vec3<T> {
    fn sub_assign(&mut self, rhs: R) {
        self[0] -= rhs.into();
        self[1] -= rhs.into();
        self[2] -= rhs.into();
    }
}

impl<T: SubAssign<T> + Copy> SubAssign<Vec3<T>> for Vec3<T> {
    fn sub_assign(&mut self, rhs: Vec3<T>) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
    }
}

impl<T: MulAssign<T> + Copy, R: Into<T> + Copy> MulAssign<R> for Vec3<T> {
    fn mul_assign(&mut self, rhs: R) {
        self[0] *= rhs.into();
        self[1] *= rhs.into();
        self[2] *= rhs.into();
    }
}

impl<T: MulAssign<T> + Copy> MulAssign<Vec3<T>> for Vec3<T> {
    fn mul_assign(&mut self, rhs: Vec3<T>) {
        self[0] *= rhs[0];
        self[1] *= rhs[1];
        self[2] *= rhs[2];
    }
}

impl<T: DivAssign<T> + Copy, R: Into<T> + Copy> DivAssign<R> for Vec3<T> {
    fn div_assign(&mut self, rhs: R) {
        self[0] /= rhs.into();
        self[1] /= rhs.into();
        self[2] /= rhs.into();
    }
}

impl<T: DivAssign<T> + Copy> DivAssign<Vec3<T>> for Vec3<T> {
    fn div_assign(&mut self, rhs: Vec3<T>) {
        self[0] /= rhs[0];
        self[1] /= rhs[1];
        self[2] /= rhs[2];
    }
}

impl<T: Copy> Vec3<T> {
    fn all(v: T) -> Vec3<T> {
        vec3![v, v, v]
    }

    fn x(&self) -> T {
        self[0]
    }

    fn y(&self) -> T {
        self[1]
    }

    fn z(&self) -> T {
        self[2]
    }

    fn xy(&self) -> Vec2<T> {
        Vec2([self.x(), self.y()])
    }

    fn yz(&self) -> Vec2<T> {
        Vec2([self.y(), self.z()])
    }
}

impl<T: Copy + Add<T, Output = T> + Mul<T, Output = T>> Vec3<T> {
    #[allow(dead_code)]
    fn dot(&self, rhs: &Vec3<T>) -> T {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }
}

impl<T: Copy + Sub<T, Output = T> + Mul<T, Output = T>> Vec3<T> {
    #[allow(dead_code)]
    fn cross(&self, rhs: &Vec3<T>) -> Vec3<T> {
        vec3![
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0]
        ]
    }
}

impl Vec3<f64> {
    fn square_length(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    fn length(&self) -> f64 {
        self.square_length().sqrt()
    }

    fn normalize(&self) -> Vec3<f64> {
        let len = self.length();
        vec3![self[0] / len, self[1] / len, self[2] / len]
    }
}

#[derive(Debug, Default, Clone)]
pub struct Vec4<T>(pub [T; 4]);

impl<T, I: SliceIndex<[T]>> Index<I> for Vec4<T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        Index::index(&self.0, index)
    }
}

impl<T, I: SliceIndex<[T]>> IndexMut<I> for Vec4<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut self.0, index)
    }
}

impl<T: PartialEq> PartialEq for Vec4<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self[0] == rhs[0] && self[1] == rhs[1] && self[2] == rhs[2] && self[3] == rhs[3]
    }
}

impl<T: Eq> Eq for Vec4<T> {}

impl<T: Add<Output = T> + Copy, R: Into<T> + Copy> Add<R> for Vec4<T> {
    type Output = Self;

    fn add(self, rhs: R) -> Self::Output {
        vec4![
            self[0] + rhs.into(),
            self[1] + rhs.into(),
            self[2] + rhs.into(),
            self[3] + rhs.into()
        ]
    }
}

impl<T: Add<Output = T> + Copy> Add<Vec4<T>> for Vec4<T> {
    type Output = Self;

    fn add(self, rhs: Vec4<T>) -> Self::Output {
        vec4![
            self[0] + rhs[0],
            self[1] + rhs[1],
            self[2] + rhs[2],
            self[3] + rhs[3]
        ]
    }
}

impl<T: Sub<Output = T> + Copy, R: Into<T> + Copy> Sub<R> for Vec4<T> {
    type Output = Self;

    fn sub(self, rhs: R) -> Self::Output {
        vec4![
            self[0] - rhs.into(),
            self[1] - rhs.into(),
            self[2] - rhs.into(),
            self[3] - rhs.into()
        ]
    }
}

impl<T: Sub<Output = T> + Copy> Sub<Vec4<T>> for Vec4<T> {
    type Output = Self;

    fn sub(self, rhs: Vec4<T>) -> Self::Output {
        vec4![
            self[0] - rhs[0],
            self[1] - rhs[1],
            self[2] - rhs[2],
            self[3] - rhs[3]
        ]
    }
}

impl<T: Mul<Output = T> + Copy, R: Into<T> + Copy> Mul<R> for Vec4<T> {
    type Output = Self;

    fn mul(self, rhs: R) -> Self::Output {
        vec4![
            self[0] * rhs.into(),
            self[1] * rhs.into(),
            self[2] * rhs.into(),
            self[3] * rhs.into()
        ]
    }
}

impl<T: Mul<Output = T> + Copy> Mul<Vec4<T>> for Vec4<T> {
    type Output = Self;

    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        vec4![
            self[0] * rhs[0],
            self[1] * rhs[1],
            self[2] * rhs[2],
            self[3] * rhs[3]
        ]
    }
}

impl<T: Div<Output = T> + Copy, R: Into<T> + Copy> Div<R> for Vec4<T> {
    type Output = Self;

    fn div(self, rhs: R) -> Self::Output {
        vec4![
            self[0] / rhs.into(),
            self[1] / rhs.into(),
            self[2] / rhs.into(),
            self[3] / rhs.into()
        ]
    }
}

impl<T: Div<Output = T> + Copy> Div<Vec4<T>> for Vec4<T> {
    type Output = Self;

    fn div(self, rhs: Vec4<T>) -> Self::Output {
        vec4![
            self[0] / rhs[0],
            self[1] / rhs[1],
            self[2] / rhs[2],
            self[3] / rhs[3]
        ]
    }
}

impl<T: AddAssign<T> + Copy, R: Into<T> + Copy> AddAssign<R> for Vec4<T> {
    fn add_assign(&mut self, rhs: R) {
        self[0] += rhs.into();
        self[1] += rhs.into();
        self[2] += rhs.into();
        self[3] += rhs.into();
    }
}

impl<T: AddAssign<T> + Copy> AddAssign<Vec4<T>> for Vec4<T> {
    fn add_assign(&mut self, rhs: Vec4<T>) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
        self[3] += rhs[3];
    }
}

impl<T: SubAssign<T> + Copy, R: Into<T> + Copy> SubAssign<R> for Vec4<T> {
    fn sub_assign(&mut self, rhs: R) {
        self[0] -= rhs.into();
        self[1] -= rhs.into();
        self[2] -= rhs.into();
        self[3] -= rhs.into();
    }
}

impl<T: SubAssign<T> + Copy> SubAssign<Vec4<T>> for Vec4<T> {
    fn sub_assign(&mut self, rhs: Vec4<T>) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
        self[3] -= rhs[3];
    }
}

impl<T: MulAssign<T> + Copy, R: Into<T> + Copy> MulAssign<R> for Vec4<T> {
    fn mul_assign(&mut self, rhs: R) {
        self[0] *= rhs.into();
        self[1] *= rhs.into();
        self[2] *= rhs.into();
        self[3] *= rhs.into();
    }
}

impl<T: MulAssign<T> + Copy> MulAssign<Vec4<T>> for Vec4<T> {
    fn mul_assign(&mut self, rhs: Vec4<T>) {
        self[0] *= rhs[0];
        self[1] *= rhs[1];
        self[2] *= rhs[2];
        self[3] *= rhs[3];
    }
}

impl<T: DivAssign<T> + Copy, R: Into<T> + Copy> DivAssign<R> for Vec4<T> {
    fn div_assign(&mut self, rhs: R) {
        self[0] /= rhs.into();
        self[1] /= rhs.into();
        self[2] /= rhs.into();
        self[3] /= rhs.into();
    }
}

impl<T: DivAssign<T> + Copy> DivAssign<Vec4<T>> for Vec4<T> {
    fn div_assign(&mut self, rhs: Vec4<T>) {
        self[0] /= rhs[0];
        self[1] /= rhs[1];
        self[2] /= rhs[2];
        self[3] /= rhs[3];
    }
}

impl<T: Copy> Vec4<T> {
    fn all(v: T) -> Vec4<T> {
        vec4![v, v, v, v]
    }

    fn x(&self) -> T {
        self[0]
    }

    fn y(&self) -> T {
        self[1]
    }

    fn z(&self) -> T {
        self[2]
    }

    fn w(&self) -> T {
        self[3]
    }

    fn xy(&self) -> Vec2<T> {
        vec2![self.x(), self.y()]
    }

    fn yz(&self) -> Vec2<T> {
        vec2![self.y(), self.z()]
    }

    fn zw(&self) -> Vec2<T> {
        vec2![self.z(), self.w()]
    }

    fn xyz(&self) -> Vec3<T> {
        vec3![self.x(), self.y(), self.z()]
    }

    fn yzw(&self) -> Vec3<T> {
        vec3![self.y(), self.z(), self.w()]
    }
}

impl<T: Copy + Add<T, Output = T> + Mul<T, Output = T>> Vec4<T> {
    #[allow(dead_code)]
    fn dot(&self, rhs: &Vec4<T>) -> T {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2] + self[3] * rhs[3]
    }
}

impl Vec4<f64> {
    fn square_length(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2] + self[3] * self[3]
    }

    fn length(&self) -> f64 {
        self.square_length().sqrt()
    }

    fn normalize(&self) -> Vec4<f64> {
        let len = self.length();
        vec4![self[0] / len, self[1] / len, self[2] / len, self[3] / len]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        assert_eq!(vec2![1, 2], vec2![1, 2]);
        assert_ne!(vec2![1, 2], vec2![100, 2]);
        assert_ne!(vec2![1, 2], vec2![1, 200]);

        assert_eq!(vec3![1, 2, 3], vec3![1, 2, 3]);
        assert_ne!(vec3![1, 2, 3], vec3![100, 2, 3]);
        assert_ne!(vec3![1, 2, 3], vec3![1, 200, 3]);
        assert_ne!(vec3![1, 2, 3], vec3![1, 2, 300]);

        assert_eq!(vec4![1, 2, 3, 4], vec4![1, 2, 3, 4]);
        assert_ne!(vec4![1, 2, 3, 4], vec4![100, 2, 3, 4]);
        assert_ne!(vec4![1, 2, 3, 4], vec4![1, 200, 3, 4]);
        assert_ne!(vec4![1, 2, 3, 4], vec4![1, 2, 300, 4]);
        assert_ne!(vec4![1, 2, 3, 4], vec4![1, 2, 3, 400]);
    }

    #[test]
    fn test_default() {
        assert_eq!(Vec2::default(), vec2![0, 0]);
        assert_eq!(Vec3::default(), vec3![0, 0, 0]);
        assert_eq!(Vec4::default(), vec4![0, 0, 0, 0]);
    }

    #[test]
    fn test_add() {
        assert_eq!(vec2![1, 2] + 3, vec2![4, 5]);
        assert_eq!(vec2![1, 2] + vec2![3, 4], vec2![4, 6]);

        assert_eq!(vec3![1, 2, 3] + 4, vec3![5, 6, 7]);
        assert_eq!(vec3![1, 2, 3] + vec3![4, 5, 6], vec3![5, 7, 9]);

        assert_eq!(vec4![1, 2, 3, 4] + 5, vec4![6, 7, 8, 9]);
        assert_eq!(vec4![1, 2, 3, 4] + vec4![5, 6, 7, 8], vec4![6, 8, 10, 12]);
    }

    #[test]
    fn test_sub() {
        assert_eq!(vec2![1, 2] - 3, vec2![-2, -1]);
        assert_eq!(vec2![1, 2] - vec2![3, 4], vec2![-2, -2]);

        assert_eq!(vec3![1, 2, 3] - 4, vec3![-3, -2, -1]);
        assert_eq!(vec3![1, 2, 3] - vec3![4, 5, 6], vec3![-3, -3, -3]);

        assert_eq!(vec4![1, 2, 3, 4] - 5, vec4![-4, -3, -2, -1]);
        assert_eq!(vec4![1, 2, 3, 4] - vec4![5, 6, 7, 8], vec4![-4, -4, -4, -4]);
    }

    #[test]
    fn test_mul() {
        assert_eq!(vec2![1, 2] * 3, vec2![3, 6]);
        assert_eq!(vec2![1, 2] * vec2![3, 4], vec2![3, 8]);

        assert_eq!(vec3![1, 2, 3] * 4, vec3![4, 8, 12]);
        assert_eq!(vec3![1, 2, 3] * vec3![4, 5, 6], vec3![4, 10, 18]);

        assert_eq!(vec4![1, 2, 3, 4] * 5, vec4![5, 10, 15, 20]);
        assert_eq!(vec4![1, 2, 3, 4] * vec4![5, 6, 7, 8], vec4![5, 12, 21, 32]);
    }

    #[test]
    fn test_div() {
        assert_eq!(vec2![12, 24] / 3, vec2![4, 8]);
        assert_eq!(vec2![12, 24] / vec2![3, 4], vec2![4, 6]);

        assert_eq!(vec3![12, 24, 32] / 4, vec3![3, 6, 8]);
        assert_eq!(vec3![12, 24, 32] / vec3![2, 3, 4], vec3![6, 8, 8]);

        assert_eq!(vec4![12, 24, 32, 40] / 4, vec4![3, 6, 8, 10]);
        assert_eq!(vec4![12, 24, 32, 40] / vec4![2, 3, 4, 5], vec4![6, 8, 8, 8]);
    }

    #[test]
    fn test_construct() {
        assert_eq!(Vec2::all(11), vec2![11, 11]);
        assert_eq!(Vec3::all(111), vec3![111, 111, 111]);
        assert_eq!(Vec4::all(1111), vec4![1111, 1111, 1111, 1111]);
    }

    #[test]
    fn test_selector() {
        assert_eq!(vec2![1, 2].x(), 1);
        assert_eq!(vec2![1, 2].y(), 2);

        assert_eq!(vec3![1, 2, 3].x(), 1);
        assert_eq!(vec3![1, 2, 3].y(), 2);
        assert_eq!(vec3![1, 2, 3].z(), 3);
        assert_eq!(vec3![1, 2, 3].xy(), vec2![1, 2]);
        assert_eq!(vec3![1, 2, 3].yz(), vec2![2, 3]);

        assert_eq!(vec4![1, 2, 3, 4].x(), 1);
        assert_eq!(vec4![1, 2, 3, 4].y(), 2);
        assert_eq!(vec4![1, 2, 3, 4].z(), 3);
        assert_eq!(vec4![1, 2, 3, 4].w(), 4);
        assert_eq!(vec4![1, 2, 3, 4].xy(), vec2![1, 2]);
        assert_eq!(vec4![1, 2, 3, 4].yz(), vec2![2, 3]);
        assert_eq!(vec4![1, 2, 3, 4].zw(), vec2![3, 4]);
        assert_eq!(vec4![1, 2, 3, 4].xyz(), vec3![1, 2, 3]);
        assert_eq!(vec4![1, 2, 3, 4].yzw(), vec3![2, 3, 4]);
    }
}
