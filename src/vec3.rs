use std::ops;

use rand::{prelude::{ThreadRng, Distribution}, Rng, distributions::Uniform};
// use num::Float;

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn zeros() -> Vec3 {
        Vec3 {x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn ones() -> Vec3 {
        Vec3 {x: 1.0, y: 1.0, z: 1.0}
    }

    pub fn random_in_unit_sphere(rng: &mut ThreadRng, uniform: Uniform<f64>) -> Vec3 {
        loop {
            let p = Vec3 {
                x: uniform.sample(rng),
                y: uniform.sample(rng),
                z: uniform.sample(rng)
            };
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector(rng: &mut ThreadRng, uniform: Uniform<f64>) -> Vec3 {
        Vec3::random_in_unit_sphere(rng, uniform).unit_vec()
    }

    pub fn random_in_hemisphere(rng: &mut ThreadRng, uniform: Uniform<f64>, normal: Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere(rng, uniform);
        if in_unit_sphere.dot(normal) > 0.0 {
            return in_unit_sphere;
        } else {
            return -in_unit_sphere;
        }
    }
}

pub trait VecLength {
    fn length(&self) -> f64;
    fn length_squared(&self) -> f64;
    fn unit_vec(self) -> Vec3;
}

impl VecLength for Vec3 {
    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    fn unit_vec(self) -> Vec3 {
        self / self.length()
    }
}

pub trait VecProducts {
    fn dot(self, vec: Vec3) -> f64;
    fn cross(self, vec: Vec3) -> Vec3;
}

impl VecProducts for Vec3 {
    fn dot(self, vec: Vec3) -> f64 {
        let multiplied_vec = self * vec;
        multiplied_vec.x + multiplied_vec.y + multiplied_vec.z
    }

    fn cross(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * vec.z - self.z * vec.y,
            y: self.z * vec.x - self.x * vec.z,
            z: self.x * vec.y - self.y * vec.x
        }
    }
}

pub trait Sqrt {
    fn sqrt(&self) -> Self;
}

impl Sqrt for Vec3 {
    fn sqrt(&self) -> Self {
        Vec3 { x: self.x.sqrt(), y: self.y.sqrt(), z: self.z.sqrt() }
    }
}

// Operator Overloads

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 { 
            x: self.x + rhs.x,
            y: self.y + rhs.y, 
            z: self.z + rhs.z
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 { 
            x: self.x - rhs.x,
            y: self.y - rhs.y, 
            z: self.z - rhs.z
        }
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        -1.0 * self
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
