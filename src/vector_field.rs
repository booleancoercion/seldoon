use crate::{Grid, Vec2};

pub struct VField<F1, F2> {
    dx: F1,
    dy: F2,
    nudge: f64,
}

impl<F1, F2> VField<F1, F2>
where
    F1: Fn(f64, f64) -> f64,
    F2: Fn(f64, f64) -> f64,
{
    pub fn new(dx: F1, dy: F2, nudge: f64) -> Self {
        Self { dx, dy, nudge }
    }

    pub fn compute(&self, pt: Vec2) -> Vec2 {
        let Vec2(x, y) = pt;

        Vec2((self.dx)(x, y), (self.dy)(x, y))
    }

    pub fn compute_normal(&self, pt: Vec2) -> Vec2 {
        self.compute(pt).normalize()
    }

    pub fn advance(&self, pt: Vec2) -> Vec2 {
        let dir = self.compute_normal(pt);

        pt + dir * self.nudge
    }

    pub fn rewind(
        &self,
        mut pt: Vec2,
        iterations: usize,
        norm_threshold: f64,
        grid: &Grid,
    ) -> Vec2 {
        for _ in 0..iterations {
            if !grid.contains(pt) {
                break;
            }
            let dir = self.compute(pt);

            if dir.norm() <= norm_threshold {
                break;
            }
            pt -= dir.normalize() * self.nudge;
        }

        pt
    }
}
