use std::{
    f64,
    cmp::PartialEq,
    ops::{Add, AddAssign, Sub, SubAssign, Div, DivAssign, Neg, Mul, MulAssign},
};

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

// this class will denote a lot of things depending on the context,
// colors with 3 coords
// points with dim coords
// vectors with dim coords
#[derive(Clone)]
pub struct VectorN {
    pub coords: Vec<f64>
}

impl VectorN {
    pub fn new(v: Vec<f64>) -> VectorN {
        VectorN {
            coords: v
        }
    }

    // difference of the length between 2 vectors
    // equivalent to distance of 2 points
    pub fn difflen(&self, v: &VectorN) -> f64 {
        (0..v.coords.len())
        .map(|i| (self.coords[i] - v.coords[i]) * (self.coords[i] - v.coords[i]))
        .sum::<f64>()
        .sqrt()
    }

    // length squared
    pub fn lensqr(&self) -> f64 {
        self.coords.iter().map(|f| f * f).sum::<f64>()
    }

    pub fn len(&self) -> f64 {
        self.lensqr().sqrt()
    }

    // unit vector colinear to initial vector
    pub fn unit(&self) -> VectorN {
        let l = self.len();
        VectorN {
            coords: self.coords.iter().map(|f| f / l).collect::<Vec<f64>>()
        }
    }

    // "stupid" multiplication, mult 1 by 1 every coord
    pub fn mult(&self, v: &VectorN) -> VectorN {
        VectorN {
            coords: (0..v.coords.len())
                        .map(|i| self.coords[i] * v.coords[i])
                        .collect::<Vec<f64>>()
        }
    }

    // same as mult
    pub fn divide(&self, v: VectorN) -> VectorN {
        VectorN {
            coords: (0..v.coords.len())
                        .map(|i| self.coords[i] / v.coords[i])
                        .collect::<Vec<f64>>()
        }
    }

    pub fn iszero(&self) -> bool {
        self.coords.iter().all(|&f| f < f64::EPSILON)
    }

    // dot product
    pub fn dot(&self, v: &VectorN) -> f64 {
        (0..v.coords.len())
            .map(|i| self.coords[i] * v.coords[i])
            .sum::<f64>()
    }

    // CAN BE USED ONLY IF THE VECTORN IS A COLOR VECTOR
    // return the color as (r, g, b) e u8^3
    pub fn get_color(&self) -> (u8, u8, u8) {
        if self.coords.len() != 3 {
            panic!("get_color on vector with a dimension != 3");
        }
        if !self.coords.iter().all(|&f| f >= 0. && f <= 1.) {
            panic!("get_color on vector with a color not in [0,1]");
        }

        return (
            (self.coords[0] * 255.999) as u8,
            (self.coords[1] * 255.999) as u8,
            (self.coords[2] * 255.999) as u8
        )
    }
}

// -- ADD --
impl Add<&VectorN> for &VectorN {
    type Output = VectorN;

    fn add(self, v: &VectorN) -> VectorN {
        VectorN {
            coords: (0..v.coords.len())
                        .map(|i| self.coords[i] + v.coords[i])
                        .collect::<Vec<f64>>()
        }
    }
}


impl Add<&VectorN> for VectorN {
    type Output = VectorN;

    fn add(self, v: &VectorN) -> VectorN {
        Self {
            coords: (0..v.coords.len())
                        .map(|i| self.coords[i] + v.coords[i])
                        .collect::<Vec<f64>>()
        }
    }
}

impl Add for VectorN {
    type Output = VectorN;

    fn add(self, v: VectorN) -> VectorN {
        VectorN {
            coords: (0..v.coords.len())
                        .map(|i| self.coords[i] + v.coords[i])
                        .collect::<Vec<f64>>()
        }
    }
}

// -- ADDASSIGN --

impl AddAssign<&VectorN> for VectorN {
    fn add_assign(&mut self, v: &VectorN) {
        for i in 0..self.coords.len() {
            self.coords[i] += v.coords[i];
        }
    }
}

impl AddAssign for VectorN {
    fn add_assign(&mut self, v: VectorN) {
        for i in 0..self.coords.len() {
            self.coords[i] += v.coords[i];
        }
    }
}

// -- SUB --

impl Sub<&VectorN> for &VectorN {
    type Output = VectorN;

    fn sub(self, v: &VectorN) -> VectorN {
        VectorN {
            coords: (0..v.coords.len())
                        .map(|i| self.coords[i] - v.coords[i])
                        .collect::<Vec<f64>>()
        }
    }
}

impl Sub<&VectorN> for VectorN {
    type Output = VectorN;

    fn sub(self, v: &VectorN) -> VectorN {
        VectorN {
            coords: (0..v.coords.len())
                        .map(|i| self.coords[i] - v.coords[i])
                        .collect::<Vec<f64>>()
        }
    }
}

impl Sub for VectorN {
    type Output = VectorN;

    fn sub(self, v: VectorN) -> VectorN {
        VectorN {
            coords: (0..v.coords.len())
                        .map(|i| self.coords[i] - v.coords[i])
                        .collect::<Vec<f64>>()
        }
    }
}

// -- SUBASSIGN --

impl SubAssign<&VectorN> for VectorN {
    fn sub_assign(&mut self, v: &VectorN) {
        for i in 0..self.coords.len() {
            self.coords[i] -= v.coords[i];
        }
    }
}

impl SubAssign for VectorN {
    fn sub_assign(&mut self, v: VectorN) {
        for i in 0..self.coords.len() {
            self.coords[i] -= v.coords[i];
        }
    }
}

// -- NEG --

impl Neg for &VectorN {
    type Output = VectorN;

    fn neg(self) -> VectorN {
        VectorN {
            coords: self.coords.iter().map(|f| -f).collect::<Vec<f64>>()
        }
    }
}

impl Neg for VectorN {
    type Output = VectorN;

    fn neg(self) -> VectorN {
        VectorN {
            coords: self.coords.iter().map(|f| -f).collect::<Vec<f64>>()
        }
    }
}

// -- MUL --

    // Cross product
impl Mul<&VectorN> for &VectorN {
    type Output = VectorN;

    fn mul(self, v: &VectorN) -> VectorN {
        let l = v.coords.len();
        VectorN {
            coords: (0..l)
                        .map(|i| self.coords[(i + 1)%l] * v.coords[(i + 2)%l]
                               - self.coords[(i + 2)%l] * v.coords[(i + 1)%l])
                        .collect::<Vec<f64>>()
        }
    }
}

impl Mul<&VectorN> for VectorN {
    type Output = VectorN;

    fn mul(self, v: &VectorN) -> VectorN {
        let l = v.coords.len();
        VectorN {
            coords: (0..l)
                        .map(|i| self.coords[(i + 1)%l] * v.coords[(i + 2)%l]
                               - self.coords[(i + 2)%l] * v.coords[(i + 1)%l])
                        .collect::<Vec<f64>>()
        }
    }
}

impl Mul<VectorN> for VectorN {
    type Output = VectorN;

    fn mul(self, v: VectorN) -> VectorN {
        let l = v.coords.len();
        VectorN {
            coords: (0..l)
                        .map(|i| self.coords[(i + 1)%l] * v.coords[(i + 2)%l]
                               - self.coords[(i + 2)%l] * v.coords[(i + 1)%l])
                        .collect::<Vec<f64>>()
        }
    }
}
    // Scaling
impl Mul<&f64> for &VectorN {
    type Output = VectorN;

    fn mul(self, s: &f64) -> VectorN {
        VectorN {
            coords: self.coords.iter().map(|f| s * f).collect::<Vec<f64>>()
        }
    }
}


impl Mul<f64> for &VectorN {
    type Output = VectorN;

    fn mul(self, s: f64) -> VectorN {
        VectorN {
            coords: self.coords.iter().map(|f| s * f).collect::<Vec<f64>>()
        }
    }
}

impl Mul<f64> for VectorN {
    type Output = VectorN;

    fn mul(self, s: f64) -> VectorN {
        VectorN {
            coords: self.coords.iter().map(|f| s * f).collect::<Vec<f64>>()
        }
    }
}

// -- MULASSIGN --

impl MulAssign<&VectorN> for VectorN {
    fn mul_assign(&mut self, v: &VectorN) {
        let l = v.coords.len();
        *self = VectorN {
            coords: (0..l)
                        .map(|i| self.coords[(i + 1)%l] * v.coords[(i + 2)%l]
                               - self.coords[(i + 2)%l] * v.coords[(i + 1)%l])
                        .collect::<Vec<f64>>()
        }
    }
}

impl MulAssign for VectorN {
    fn mul_assign(&mut self, v: VectorN) {
        let l = v.coords.len();
        *self = VectorN {
            coords: (0..l)
                        .map(|i| self.coords[(i + 1)%l] * v.coords[(i + 2)%l]
                               - self.coords[(i + 2)%l] * v.coords[(i + 1)%l])
                        .collect::<Vec<f64>>()
        }
    }
}

impl MulAssign<f64> for VectorN {
    fn mul_assign(&mut self, f: f64) {
        for n in self.coords.iter_mut() {
            *n *= f
        }
    }
}

// -- DIV --

    // scaling
impl Div<&f64> for &VectorN {
    type Output = VectorN;

    fn div(self, s: &f64) -> VectorN {
        VectorN {
            coords: self.coords.iter().map(|f| f / s).collect::<Vec<f64>>()
        }
    }
}


impl Div<&f64> for VectorN {
    type Output = VectorN;

    fn div(self, s: &f64) -> VectorN {
        VectorN {
            coords: self.coords.iter().map(|f| f / s).collect::<Vec<f64>>()
        }
    }
}


impl Div<f64> for VectorN {
    type Output = VectorN;

    fn div(self, s: f64) -> VectorN {
        VectorN {
            coords: self.coords.iter().map(|f| f / s).collect::<Vec<f64>>()
        }
    }
}

// -- DIVASSIGN --

impl DivAssign<f64> for VectorN {
    fn div_assign(&mut self, f: f64) {
        for n in self.coords.iter_mut() {
            *n /= f
        }
    }
}


// euh

impl PartialEq for VectorN {
    fn eq(&self, v: &VectorN) -> bool {
        (0..v.coords.len()).all(|i| v.coords[i] == self.coords[i])
    }
}


/* ---------------------TESTS--------------------- */

#[test]
fn test_gen3() {
    let v = VectorN {
        coords: vec![0., 1., 3.]
    };

    let s1 = vec![0., 1., 3.];
    for i in 0..s1.len() {
        assert_approx_eq!(s1[i], v.coords[i]);
    }
    assert!(s1.len() == v.coords.len());

    let w = VectorN::new(vec![98., 0.81, 42.69]);

    let s2 = vec![98., 0.81, 42.69];
    for i in 0..s2.len() {
        assert_approx_eq!(s2[i], w.coords[i]);
    }
    assert!(s2.len() == w.coords.len());
}

#[test]
fn test_difflen3() {
    let v = VectorN::new(vec![0.1, 0.2, 0.3]);
    let w = VectorN::new(vec![0.4, 0.3, 0.2]);

    assert_approx_eq!(v.difflen(&w), (0.11 as f64).sqrt());
}

#[test]
fn test_lensqr3() {
    let v = VectorN::new(vec![0.1, 0.2, 0.3]);

    assert_approx_eq!(v.lensqr(), 0.14);
}

#[test]
fn test_len3() {
    let v = VectorN::new(vec![0.1, 0.2, 0.3]);

    assert_approx_eq!(v.len(), (0.14 as f64).sqrt())
}

#[test]
fn test_unit3() {
    let v = VectorN::new(vec![0.1, 0.2, 0.3]);
    let r = v.unit();

    let l = v.len();
    let s = vec![0.1/l, 0.2/l, 0.3/l];
    for i in 0..s.len()
    {
        assert_approx_eq!(s[i], r.coords[i]);
    }
    assert!(s.len() == r.coords.len());
    assert_approx_eq!(1., r.len());
}

#[test]
fn test_mult3() {
    let v = VectorN::new(vec![0.1, 0.2, 0.3]);
    let w = VectorN::new(vec![0.2, 0.3, 0.4]);
    let r = v.mult(&w);

    let s = vec![0.02, 0.06, 0.12];
    for i in 0..s.len()
    {
        assert_approx_eq!(s[i], r.coords[i]);
    }
    assert!(s.len() == w.coords.len());
}

#[test]
fn test_divide3() {
    let v = VectorN::new(vec![0.1, 0.2, 0.3]);
    let w = VectorN::new(vec![0.2, 0.3, 0.4]);
    let r = v.divide(w);

    let s = vec![0.5, 2./3., 0.3/0.4];
    for i in 0..s.len()
    {
        assert_approx_eq!(s[i], r.coords[i]);
    }
    assert!(s.len() == r.coords.len());
}

#[test]
fn test_iszero3() {
    let v = VectorN::new(vec![0.1, 0.2, 0.3]);
    assert!(!v.iszero());
    let w = VectorN::new(vec![0.0, 0.0, 0.0]);
    assert!(w.iszero());
}

#[test]
fn test_dot3() {
    let v = VectorN::new(vec![0.1, 0.2, 0.3]);
    let w = VectorN::new(vec![0.2, 0.3, 0.4]);

    assert_approx_eq!(v.dot(&w), 0.2);
}

#[test]
fn test_get_color() {
    let v = VectorN::new(vec![1., 1., 1.]);
    let (r1, g1, b1) = v.get_color();

    assert!(r1 == 255);
    assert!(g1 == 255);
    assert!(b1 == 255);

    let w = VectorN::new(vec![0., 0., 0.]);
    let (r2, g2, b2) = w.get_color();
    assert!(r2 == 0);
    assert!(g2 == 0);
    assert!(b2 == 0);

    let vvv = VectorN::new(vec![0.1, 0.2, 0.3]);
    let (r3, g3, b3) = vvv.get_color();
    assert!(r3 == 25);
    assert!(g3 == 51);
    assert!(b3 == 76);
}

#[test]
fn test_add3() {
    let v = VectorN::new(vec![0.1, 0.2, 0.3]);
    let w = VectorN::new(vec![0.2, 0.3, 0.4]);
    let r = v + w;

    let s = vec![0.3, 0.5, 0.7];
    for i in 0..s.len() {
        assert_approx_eq!(s[i], r.coords[i]);
    }
    assert!(s.len() == r.coords.len());
}

#[test]
fn test_sub3() {
    let v = VectorN::new(vec![0.1, 0.2, 0.3]);
    let w = VectorN::new(vec![0.2, 0.3, 0.4]);
    let r = v - w;

    let s = vec![-0.1, -0.1, -0.1];
    for i in 0..s.len() {
        assert_approx_eq!(s[i], r.coords[i])
    }
    assert!(s.len() == r.coords.len());
}

#[test]
fn test_neg3() {
    let v = VectorN::new(vec![0.1, 0.2, 0.3]);
    let r = -v;

    let s = vec![-0.1, -0.2, -0.3];
    for i in 0..s.len()
    {
        assert_approx_eq!(s[i], r.coords[i]);
    }
    assert!(s.len() == r.coords.len());
}

#[test]
fn test_cross3() {
    let v = VectorN::new(vec![0.1, 0.2, 0.3]);
    let w = VectorN::new(vec![0.2, 0.3, 0.4]);
    let r = v * &w;

    let s = vec![-0.01, 0.02, -0.01];
    for i in 0..s.len()
    {
        assert_approx_eq!(s[i], r.coords[i]);
    }
    assert!(s.len() == r.coords.len());
}

#[test]
fn test_scalar_mul3() {
    let v = VectorN::new(vec![0.1, 0.2, 0.3]);
    let r = v * 42.;

    let s = vec![4.2, 8.4, 12.6];
    for i in 0..s.len()
    {
        assert_approx_eq!(s[i], r.coords[i]);
    }
    assert!(s.len() == r.coords.len());
}

#[test]
fn test_scalar_div3() {
    let v = VectorN::new(vec![0.1, 0.2, 0.3]);
    let r = v / 42.;

    let s = vec![0.1/42., 0.2/42., 0.3/42.];
    for i in 0..s.len()
    {
        assert_approx_eq!(s[i], r.coords[i]);
    }
    assert!(s.len() == r.coords.len());
}

#[test]
fn test_eq3() {
    let v = VectorN::new(vec![0.2, 0.3, 0.4]);
    let w = VectorN::new(vec![0.1, 0.2, 0.3]);
    let vv = VectorN::new(vec![0.1, 0.2, 0.3]);

    assert!(!(v == w));
    assert!(w == vv);
    assert!(!(vv == v));
}
