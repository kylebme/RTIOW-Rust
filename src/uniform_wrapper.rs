use rand::{
    distributions::{DistIter, Distribution, Uniform},
    prelude::{Rng, ThreadRng, thread_rng},
};

pub trait UniGen {
    fn sample(&mut self) -> f64;
}


pub struct UniGen0_1 {
    dist_iter: DistIter<Uniform<f64>, ThreadRng, f64>,
}

impl UniGen0_1 {
    pub fn new() -> Self {
        let uniform = Uniform::new(0.0, 1.0);
        let rng = thread_rng();
        Self {
            dist_iter: uniform.sample_iter(rng),
        }
    }
}

impl UniGen for UniGen0_1 {
    fn sample(&mut self) -> f64 {
        // dist_iter.next always returns Some, so hopefully this should never panic
        self.dist_iter.next().unwrap()
    }
}

pub struct UniGenNeg1_1 {
    dist_iter: DistIter<Uniform<f64>, ThreadRng, f64>,
}

impl UniGenNeg1_1 {
    pub fn new() -> Self {
        let uniform = Uniform::new(-1.0, 1.0);
        let rng = thread_rng();
        Self {
            dist_iter: uniform.sample_iter(rng),
        }
    }
}

impl UniGen for UniGenNeg1_1 {
    fn sample(&mut self) -> f64 {
        // dist_iter.next always returns Some, so hopefully this should never panic
        self.dist_iter.next().unwrap()
    }
}


pub struct UniGenUntyped {
    dist_iter: DistIter<Uniform<f64>, ThreadRng, f64>,
}

impl UniGenUntyped {
    pub fn new(start: f64, end: f64) -> Self {
        let uniform = Uniform::new(start, end);
        let rng = thread_rng();
        Self {
            dist_iter: uniform.sample_iter(rng),
        }
    }
}

impl UniGen for UniGenUntyped {
    fn sample(&mut self) -> f64 {
        // dist_iter.next always returns Some, so hopefully this should never panic
        self.dist_iter.next().unwrap()
    }
}

// pub struct UniformWrapper<T: SampleUniform + Copy> {
//     uniform: Uniform<T>,
//     range: Range<T>,
// }

// impl<T: SampleUniform + Copy> Copy for UniformWrapper<T> {}

// impl<T: SampleUniform + Copy> Clone for UniformWrapper<T> {
//     fn clone(&self) -> UniformWrapper<T> {
//         *self
//     }
// }

// impl<T: SampleUniform + Copy> UniformWrapper<T> {
//     fn range(self) -> Range<T> {
//         self.range
//     }
// }

// impl<T: SampleUniform + Copy> From<Range<T>> for UniformWrapper<T> {
//     fn from(r: Range<T>) -> UniformWrapper<T> {
//         UniformWrapper {
//             uniform: Uniform::new(r.start, r.end),
//             range: r,
//         }
//     }
// }

// impl<T: SampleUniform + Copy> Distribution<T> for UniformWrapper<T> {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T {
//         self.uniform.sample(rng)
//     }
// }
