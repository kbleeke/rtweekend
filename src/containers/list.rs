use crate::hit::{surrounding_box, Aabb, HitRecord, Hitable, Ray};

impl<T> Hitable for Vec<T>
where
    T: Hitable,
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.as_slice().hit(r, t_min, t_max)
    }

    fn bounding_box(&self) -> Aabb {
        self.as_slice().bounding_box()
    }
}

impl<T, const N: usize> Hitable for [T; N]
where
    T: Hitable,
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.as_ref().hit(r, t_min, t_max)
    }

    fn bounding_box(&self) -> Aabb {
        self.as_ref().bounding_box()
    }
}

impl<const N: usize> Hitable for [Box<dyn Hitable>; N] {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.as_ref().hit(r, t_min, t_max)
    }

    fn bounding_box(&self) -> Aabb {
        self.as_ref().bounding_box()
    }
}

impl<T> Hitable for [T]
where
    T: Hitable,
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;

        let mut hit_anything = None;

        for hitable in self {
            if let Some(hit_rec) = hitable.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_rec.t;
                hit_anything = Some(hit_rec);
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> Aabb {
        assert!(!self.is_empty(), "hitable list must not be empty");

        let mut it = self.iter();
        let first = it.next().unwrap().bounding_box();
        it.fold(first, |b, h| surrounding_box(&b, &h.bounding_box()))
    }
}

impl Hitable for [Box<dyn Hitable>] {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;

        let mut hit_anything = None;

        for hitable in self {
            if let Some(hit_rec) = hitable.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_rec.t;
                hit_anything = Some(hit_rec);
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> Aabb {
        assert!(self.len() > 0, "hitable list must not be empty");

        let mut it = self.iter();
        let first = it.next().unwrap().bounding_box();
        it.fold(first, |b, h| surrounding_box(&b, &h.bounding_box()))
    }
}
