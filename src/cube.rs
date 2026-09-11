use crate::ray_intersect::{Intersect, Material, RayIntersect};
use nalgebra_glm::Vec3;

const EPSILON: f32 = 1e-4;

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
}

impl Cube {
    pub fn new(min: Vec3, max: Vec3, material: Material) -> Self {
        Cube { min, max, material }
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        let mut near = -f32::INFINITY;
        let mut far = f32::INFINITY;

        for axis in 0..3 {
            let origin = ray_origin[axis];
            let direction = ray_direction[axis];
            let minimum = self.min[axis];
            let maximum = self.max[axis];

            if direction.abs() < EPSILON {
                if origin < minimum || origin > maximum {
                    return None;
                }
                continue;
            }

            let mut axis_near = (minimum - origin) / direction;
            let mut axis_far = (maximum - origin) / direction;

            if axis_near > axis_far {
                std::mem::swap(&mut axis_near, &mut axis_far);
            }

            near = near.max(axis_near);
            far = far.min(axis_far);

            if near > far {
                return None;
            }
        }

        let distance = if near > EPSILON { near } else { far };
        if distance <= EPSILON {
            return None;
        }

        let point = ray_origin + ray_direction * distance;
        let normal = if (point.x - self.min.x).abs() < EPSILON {
            Vec3::new(-1.0, 0.0, 0.0)
        } else if (point.x - self.max.x).abs() < EPSILON {
            Vec3::new(1.0, 0.0, 0.0)
        } else if (point.y - self.min.y).abs() < EPSILON {
            Vec3::new(0.0, -1.0, 0.0)
        } else if (point.y - self.max.y).abs() < EPSILON {
            Vec3::new(0.0, 1.0, 0.0)
        } else if (point.z - self.min.z).abs() < EPSILON {
            Vec3::new(0.0, 0.0, -1.0)
        } else {
            Vec3::new(0.0, 0.0, 1.0)
        };

        Some(Intersect {
            point,
            normal,
            distance,
            material: self.material,
        })
    }
}