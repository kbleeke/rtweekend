use std::cmp::Ordering;

use rand::{thread_rng, Rng};

use super::{
    aabb::surrounding_box,
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct BvhNode {
    bbox: Aabb,
    left: Option<Box<dyn Hittable>>,
    right: Option<Box<dyn Hittable>>,
}

impl BvhNode {
    pub fn new(mut hittables: Vec<Box<dyn Hittable>>, time0: f32, time1: f32) -> Self {
        let axis = thread_rng().gen_range(0..3);
        if axis == 0 {
            hittables.sort_unstable_by(|a, b| box_x_compare(a.as_ref(), b.as_ref()));
        } else if axis == 1 {
            hittables.sort_unstable_by(|a, b| box_y_compare(a.as_ref(), b.as_ref()));
        } else {
            hittables.sort_unstable_by(|a, b| box_z_compare(a.as_ref(), b.as_ref()));
        }

        assert!(!hittables.is_empty());
        if hittables.len() == 1 {
            let left = hittables.pop().unwrap();
            let bbox = left
                .bounding_box(time0, time1)
                .expect("no bounding box in BvhNode::new");
            Self {
                bbox,
                left: Some(left),
                right: None,
            }
        } else if hittables.len() == 2 {
            let left = hittables.pop().unwrap();
            let right = hittables.pop().unwrap();

            let bbox_left = left
                .bounding_box(time0, time1)
                .expect("no bounding box in BvhNode::new");
            let bbox_right = right
                .bounding_box(time0, time1)
                .expect("no bounding box in BvhNode::new");
            let bbox = surrounding_box(&bbox_left, &bbox_right);

            Self {
                bbox,
                left: Some(left),
                right: Some(right),
            }
        } else {
            let second = hittables.split_off(hittables.len() / 2);
            let left = Box::new(BvhNode::new(hittables, time0, time1));
            let right = Box::new(BvhNode::new(second, time0, time1));

            let bbox_left = left
                .bounding_box(time0, time1)
                .expect("no bounding box in BvhNode::new");
            let bbox_right = right
                .bounding_box(time0, time1)
                .expect("no bounding box in BvhNode::new");
            let bbox = surrounding_box(&bbox_left, &bbox_right);

            Self {
                bbox,
                left: Some(left),
                right: Some(right),
            }
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        if self.bbox.hit(r, t_min, t_max) {
            let left = self.left.as_ref().and_then(|h| h.hit(r, t_min, t_max));
            let right = self.right.as_ref().and_then(|h| h.hit(r, t_min, t_max));
            match (left, right) {
                (Some(left_rec), Some(right_rec)) if left_rec.t < right_rec.t => Some(left_rec),
                (_, Some(right_rec)) => Some(right_rec),
                (Some(left_rec), None) => Some(left_rec),
                (None, None) => None,
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
        Some(self.bbox)
    }
}

fn box_x_compare(a: &dyn Hittable, b: &dyn Hittable) -> Ordering {
    let a_box = a
        .bounding_box(0.0, 0.0)
        .expect("no boundingbox in BvhNode::new");
    let b_box = b
        .bounding_box(0.0, 0.0)
        .expect("no boundingbox in BvhNode::new");

    if a_box.min().x - b_box.min().x < 0.0 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn box_y_compare(a: &dyn Hittable, b: &dyn Hittable) -> Ordering {
    let a_box = a
        .bounding_box(0.0, 0.0)
        .expect("no boundingbox in BvhNode::new");
    let b_box = b
        .bounding_box(0.0, 0.0)
        .expect("no boundingbox in BvhNode::new");

    if a_box.min().y - b_box.min().y < 0.0 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn box_z_compare(a: &dyn Hittable, b: &dyn Hittable) -> Ordering {
    let a_box = a
        .bounding_box(0.0, 0.0)
        .expect("no boundingbox in BvhNode::new");
    let b_box = b
        .bounding_box(0.0, 0.0)
        .expect("no boundingbox in BvhNode::new");

    if a_box.min().z - b_box.min().z < 0.0 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}
