use std::f64;
use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Clone)]
pub struct VectorN {
    coords: Vec<f64>
}

impl VectorN {
    pub fn difflen(&self, v: &VectorN) -> f64 {
        let mut tot = 0.;
        for i in 0..v.coords.len() {
            tot += self.coords[i] - v.coords[i];
        }

        tot.sqrt()
    }

    pub fn len(&self) -> f64 {
        self.coords.iter().map(|f| f * f).sum::<f64>().sqrt()
    }

    pub fn lensqr(&self) -> f64 {
        self.coords.iter().map(|f| f * f).sum::<f64>()
    }

    pub fn unit(&self) -> VectorN {
        let l = self.len();
        VectorN {
            coords: self.coords.iter().map(|f| f / l).collect::<Vec<f64>>()
        }
    }

    pub fn mul(&self, v: VectorN) -> VectorN {
        VectorN {
            coords: (0..v.coords.len())
                        .map(|i| self.coords[i] * v.coords[i])
                        .collect::<Vec<f64>>()
        }
    }

    pub fn div(&self, v: VectorN) -> VectorN {
        VectorN {
            coords: (0..v.coords.len())
                        .map(|i| self.coords[i] / v.coords[i])
                        .collect::<Vec<f64>>()
        }
    }

    pub fn iszero(&self) -> bool {
        self.coords.iter().all(|&f| f < f64::EPSILON)
    }

    pub fn dot(&self, v: &VectorN) -> f64 {
        (0..v.coords.len())
            .map(|i| self.coords[i] * v.coords[i])
            .sum::<f64>()
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

impl Neg for VectorN {
    type Output = VectorN;

    fn neg(self) -> VectorN {
        VectorN {
            coords: self.coords.iter().map(|f| -f).collect::<Vec<f64>>()
        }
    }
}

impl Mul<VectorN> for VectorN {
    type Output = VectorN;

    fn mul(self, v: VectorN) -> VectorN {
        let l = v.coords.len();
        VectorN {
            coords: (0..l)
                        .map(|i| self.coords[(i + 1)%l] * v.coords[(i + 2)%l])
                        .collect::<Vec<f64>>()
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

impl Div<f64> for VectorN {
    type Output = VectorN;

    fn div(self, s: f64) -> VectorN {
        VectorN {
            coords: self.coords.iter().map(|f| f / s).collect::<Vec<f64>>()
        }
    }
}

impl PartialEq for VectorN {
    fn eq(&self, v: &VectorN) -> bool {
        (0..v.coords.len()).all(|i| v.coords[i] == self.coords[i])
    }
}
