use std::ops::*;

struct Pouet {
    attr: Vec<i32>,
    len: i32,
}

impl Add<&Pouet> for &Pouet {
    type Output = Pouet;

    fn add(self, other: &Pouet) -> Pouet {
        Pouet {
            attr: self.attr.iter().copied().chain(other.attr.iter().copied()).collect::<Vec<i32>>(),
            len: self.len + other.len,
        }
    }
}

impl Add<&Pouet> for Pouet {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        Self {
            attr: [self.attr, other.attr.clone()].concat(),
            len: self.len + other.len,
        }
    }
}

impl Add<Pouet> for Pouet {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            attr: [self.attr, other.attr].concat(),
            len: self.len + other.len,
        }
    }
}

impl Pouet {
    fn test(&mut self) {
        for i in 0..self.len as usize {
            self.attr[i] += self.len;
        }
    }
}

fn main() {
    let a = Pouet {attr: vec![0,1,2], len: 3};
    let b = Pouet {attr: vec![10,11,12,13], len: 4};
    let c = &a + &b;

    println!("{:?}", c.attr);
}
