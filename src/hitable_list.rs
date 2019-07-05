use crate::ray::*;

pub struct HitableList {
    list: Vec<Box<dyn Hitable + Send>>,
}

impl HitableList {
    pub fn new(list: Vec<Box<dyn Hitable + Send>>) -> Self {
        Self { list }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for hitable in &self.list {
            if let Some(temp_rec) = hitable.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }
        rec
    }
}

trait OptionExt<T>: Sized {
    fn inspect<F>(self, f: F) -> Self where F: FnMut(&mut T);
}

impl<T> OptionExt<T> for Option<T> {
    fn inspect<F>(mut self, mut f: F) -> Self where F: FnMut(&mut T) {
        if let Some(ref mut t) = self {
            f(t);
        }
        self
    }
}
