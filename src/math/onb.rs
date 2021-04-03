use super::{cross, vec3, Vec3};

pub struct Onb {
    axis: [Vec3; 3],
}

impl Onb {
    pub fn build_from(w: &Vec3) -> Self {
        let w = w.normalize();
        let a = if w.x().abs() > 0.9 {
            vec3(0, 1, 0)
        } else {
            vec3(1, 0, 0)
        };
        let v = cross(&w, &a).normalize();
        let u = cross(&w, &v);

        Self { axis: [u, v, w] }
    }

    pub fn u(&self) -> &Vec3 {
        &self.axis[0]
    }

    pub fn v(&self) -> &Vec3 {
        &self.axis[1]
    }

    pub fn w(&self) -> &Vec3 {
        &self.axis[2]
    }

    pub fn local(&self, a: &Vec3) -> Vec3 {
        a.x() * self.u() + a.y() * self.v() + a.z() * self.w()
    }
}
