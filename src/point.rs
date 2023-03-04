#![allow(dead_code)]

use crate::vec::{Vec2, Vec3};
use std::ops::{Add, AddAssign, Index, IndexMut};
use std::slice::SliceIndex;


#[macro_export]
macro_rules! point2 {
    ($x:expr, $y:expr) => {
        Point2::new($x, $y)
    };
}

#[macro_export]
macro_rules! point3 {
    ($x:expr, $y:expr, $z:expr) => {
        Point3::new($x, $y, $z)
    };
}

pub struct Point2(pub [f64; 2]);

impl Point2 {
    fn new(x: f64, y: f64) -> Point2 {
        Point2([x, y])
    }

    fn x(&self) -> f64 {
        self[0]
    }

    fn y(&self) -> f64 {
        self[1]
    }

    fn set_x(&mut self, x: f64) {
        self[0] = x;
    }

    fn set_y(&mut self, y: f64) {
        self[1] = y;
    }

    fn set(&mut self, x: f64, y: f64) {
        self[0] = x;
        self[1] = y;
    }
}

impl<I: SliceIndex<[f64]>> Index<I> for Point2 {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        Index::index(&self.0, index)
    }
}

impl<I: SliceIndex<[f64]>> IndexMut<I> for Point2 {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut self.0, index)
    }
}

impl Add<Vec2<f64>> for Point2 {
    type Output = Self;

    fn add(self, rhs: Vec2<f64>) -> Self::Output {
        point2![self[0] + rhs[0], self[1] + rhs[1]]
    }
}

impl AddAssign<Vec2<f64>> for Point2 {
    fn add_assign(&mut self, rhs: Vec2<f64>) {
        self[0] += rhs[0];
        self[1] += rhs[1];
    }
}


pub struct Point3(pub [f64; 3]);

impl Point3 {
    fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3([x, y, z])
    }

    fn x(&self) -> f64 {
        self[0]
    }

    fn y(&self) -> f64 {
        self[1]
    }

    fn z(&self) -> f64 {
        self[2]
    }

    fn set_x(&mut self, x: f64) {
        self[0] = x;
    }

    fn set_y(&mut self, y: f64) {
        self[1] = y;
    }

    fn set_z(&mut self, z: f64) {
        self[2] = z;
    }

    fn set(&mut self, x: f64, y: f64, z: f64) {
        self[0] = x;
        self[1] = y;
        self[2] = z;
    }
}

impl<I: SliceIndex<[f64]>> Index<I> for Point3 {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        Index::index(&self.0, index)
    }
}

impl<I: SliceIndex<[f64]>> IndexMut<I> for Point3 {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut self.0, index)
    }
}

impl Add<Vec3<f64>> for Point3 {
    type Output = Self;

    fn add(self, rhs: Vec3<f64>) -> Self::Output {
        Point3::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl AddAssign<Vec3<f64>> for Point3 {
    fn add_assign(&mut self, rhs: Vec3<f64>) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}