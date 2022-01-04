use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3
}

pub trait VecAt {
    fn at(self, t: f64) -> Vec3;
}

impl VecAt for Ray {
    fn at(self, t: f64) -> Vec3 {
        self.orig + t * self.dir
    }
}
