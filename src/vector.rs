use std::f64;
use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug, Clone, Copy)]
pub struct Vector3
{
    x: f64,
    y: f64,
    z: f64
}

impl Vector3
{
    pub fn difflen(&self, v: &Vector3) -> f64
    {
        (self.x - v.x) * (self.x - v.x)
        + (self.y - v.y) * (self.y - v.y)
        + (self.z - v.z) * (self.z - v.z)
    }

    pub fn len(&self) -> f64
    {
        (self.x * self.x
        + self.y * self.y
        + self.z * self.z).sqrt()
    }

    pub fn lensqrd(&self) -> f64
    {
        self.x * self.x
        + self.y * self.y
        + self.z * self.z
    }

    pub fn unit(&self) -> Vector3
    {
        Vector3 {
            x: self.x / self.len(),
            y: self.y / self.len(),
            z: self.z / self.len()
        }
    }

    pub fn dot(&self, v: &Vector3) -> f64
    {
        self.x * v.x
        + self.y * v.y
        + self.z * v.z
    }

    pub fn cross(&self, v: &Vector3) -> Vector3
    {
        Vector3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    pub fn iszero(&self) -> bool
    {
        self.x.abs() < f64::EPSILON
        && self.y.abs() < f64::EPSILON
        && self.z.abs() < f64::EPSILON

    }
}

impl Add for Vector3
{
    type Output = Vector3;

    fn add(self, v: Vector3) -> Vector3
    {
        Vector3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, v: Vector3) -> Vector3
    {
        Vector3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3
    {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, v: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, s: f64) -> Vector3 {
        Vector3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

impl Div<Vector3> for Vector3 {
    type Output = Vector3;

    fn div(self, v: Vector3) -> Vector3
    {
        Vector3 {
            x: self.x / v.x,
            y: self.y / v.y,
            z: self.z / v.z,
        }
    }
}

impl Div<f64> for Vector3
{
    type Output = Vector3;

    fn div(self, s: f64) -> Vector3
    {
        Vector3 {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s,
        }
    }
}

impl PartialEq for Vector3
{
    fn eq(&self, v: &Vector3) -> bool
    {
        self.x == v.x
        && self.y == v.y
        && self.z == v.z
    }
}
