use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct vector3
{
    x: f64,
    y: f64,
    z: f64
}

impl vector3
{
    pub fn difflen(&self, v: &vector3) -> f64
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

    pub fn unit(&self) -> vector3
    {
        vector3::new(
            self.x / self.len(),
            self.y / self.len(),
            self.z / self.len()
        )
    }

    pub fn dot(&self, v: &vector3)
    {
        self.x * v.x
        + self.y * v.y
        + self.z * v.z
    }

    pub fn cross(&self, v: &vector3)
    {
        vector3::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }

    pub fn iszero(&self) -> bool
    {
        self.x.abs() < f64::EPSILON
        && self.y.abs() < f64::EPSILON
        && self.z.abs() < f64::EPSILON

    }
}

impl Add for vector3
{
    type Output = vector3;

    fn add(self, v: vector3) -> vector3
    {
        vector3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl Sub for vector3 {
    type Output = vector3;

    fn sub(self, v: vector3) -> vector3
    {
        vector3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}

impl Neg for vector3 {
    type Output = vector3;

    fn neg(self) -> vector3
    {
        vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<vector3> for vector3 {
    type Output = vector3;

    fn mul(self, v: vector3) -> vector3 {
        vector3 {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }
}

impl Mul<f64> for vector3 {
    type Output = vector3;

    fn mul(self, s: f64) -> vector3 {
        vector3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

impl Div<vector3> for vector3 {
    type Output = vector3;

    fn div(self, v: vector3) -> vector3
    {
        vector3 {
            x: self.x / v.x,
            y: self.y / v.y,
            z: self.z / v.z,
        }
    }
}

impl Div<f64> for vector3
{
    type Output = vector3;

    fn div(self, s: f64) -> vector3
    {
        vector3 {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s,
        }
    }
}

impl PartialEq for vector3
{
    fn eq(&self, v: &vector3) -> bool
    {
        self.x == v.x
        && self.y == v.y
        && self.z == v.z
    }
}
