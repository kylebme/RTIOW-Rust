use std::ops;

use crate::uniform_wrapper::*;
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

    pub fn random_in_unit_sphere(unigen: &mut impl UniGen) -> Vec3 {
        loop {
            let p = Vec3 {
                x: unigen.sample(),
                y: unigen.sample(),
                z: unigen.sample()
            };
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector(unigen: &mut impl UniGen) -> Vec3 {
        Vec3::random_in_unit_sphere(unigen).unit_vec()
    }

    pub fn random_in_hemisphere(unigen: &mut UniGenNeg1_1, normal: Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere(unigen);
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    // Should add some strong typing to this random range
    pub fn random_in_unit_disk(unigen: &mut UniGenNeg1_1) -> Vec3 {
        loop {
            let p = Vec3 {
                x: unigen.sample(),
                y: unigen.sample(),
                z: 0.0
            };
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}

pub trait VecLength {
    fn length(&self) -> f64;
    fn length_squared(&self) -> f64;
    fn unit_vec(self) -> Vec3;
    fn near_zero(&self) -> bool;
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

    fn near_zero(&self) -> bool {
        let epsilon = 1e-8;
        (self.x < epsilon) && (self.y < epsilon) && (self.z < epsilon)
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

pub trait Reflect {
    fn reflect(self, normal: Vec3) -> Vec3;
}

impl Reflect for Vec3 {
    fn reflect(self, normal: Vec3) -> Vec3 {
        self - 2.0 * self.dot(normal) * normal
    }
}

pub trait Refract {
    fn refract(self, normal: Vec3, etai_over_etat: f64) -> Vec3;
}

impl Refract for Vec3 {
    fn refract(self, normal: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = ((-self).dot(normal)).min(1.0);
        let r_out_perpendicular = etai_over_etat * (self + cos_theta * normal);
        let r_out_parallel =  -1.0 * normal * (1.0 - r_out_perpendicular.length_squared()).abs().sqrt();
        r_out_perpendicular + r_out_parallel
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
