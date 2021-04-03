use crate::hit::{Aabb, Hitable};
use crate::hit::{HitRecord, Ray};

pub struct BvhNode {
    bbox: Aabb,
    left: Option<Box<dyn Hitable>>,
    right: Option<Box<dyn Hitable>>,
}

impl Hitable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.bbox
            .hit(r, t_min, t_max)
            .then(|| {
                let hit_left = self.left.as_ref().and_then(|h| h.hit(r, t_min, t_max));
                let hit_right = self.right.as_ref().and_then(|h| h.hit(r, t_min, t_max));
                match (hit_left, hit_right) {
                    (Some(left_rec), Some(right_rec)) => {
                        if left_rec.t < right_rec.t {
                            Some(left_rec)
                        } else {
                            Some(right_rec)
                        }
                    }
                    (Some(left_rec), _) => Some(left_rec),
                    (_, Some(right_rec)) => Some(right_rec),
                    _ => None,
                }
            })
            .flatten()
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
