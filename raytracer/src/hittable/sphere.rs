use std::f64::consts::PI;
use std::f64::INFINITY;

use crate::basic::onb::Onb;
use crate::basic::ray::Ray;
use crate::basic::vec::{random_to_sphere, Point3, Vec3};
use crate::hittable::bvh::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;

#[derive(Clone)]
pub struct Sphere<M>
where
    M: Material + Clone,
{
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: M,
}

impl<M: Material + Clone> Sphere<M> {
    pub fn new(center: Point3, radius: f64, mat_ptr: M) -> Self {
        Self {
            center,
            radius,
            mat_ptr,
        }
    }

    fn get_sphere_uv(&self, p: Point3) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = f64::atan2(-p.z, p.x) + PI;
        (phi / (2. * PI), theta / PI)
    }
}

impl<M: Material + Clone> Hittable for Sphere<M> {
    #[allow(clippy::many_single_char_names)]
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length_sqr();
        let half_b = Vec3::dot(oc, r.dir);
        let c = oc.length_sqr() - self.radius * self.radius;

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let outward_normal = (r.at(root) - self.center) / self.radius;
        let (u, v) = self.get_sphere_uv(outward_normal);
        let mut rec = HitRecord::new(r.at(root), outward_normal, root, u, v, false, &self.mat_ptr);

        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }

    fn pdf_value(&self, o: Point3, v: Vec3) -> f64 {
        if let Some(_rec) = self.hit(Ray::new(o, v, 0.), 0.001, INFINITY) {
            let cos_max = (1. - self.radius * self.radius / (self.center - o).length_sqr()).sqrt();
            let solid_angle = 2. * PI * (1. - cos_max);
            1. / solid_angle
        } else {
            0.
        }
    }

    fn random(&self, o: Point3) -> Vec3 {
        let direction = self.center - o;
        let dis_sqr = direction.length_sqr();
        let uvw = Onb::build_from_w(direction);
        uvw.local_vec(random_to_sphere(self.radius, dis_sqr))
    }
}

pub struct MovingSphere<M>
where
    M: Material,
{
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: M,
}
impl<M: Material> MovingSphere<M> {
    #[allow(dead_code)]
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        mat_ptr: M,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            mat_ptr,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }

    fn get_sphere_uv(&self, p: Point3) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = f64::atan2(-p.z, p.x) + PI;
        (phi / (2. * PI), theta / PI)
    }
}

impl<M: Material> Hittable for MovingSphere<M> {
    #[allow(clippy::many_single_char_names)]
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center(r.tm);
        let a = r.dir.length_sqr();
        let half_b = Vec3::dot(oc, r.dir);
        let c = oc.length_sqr() - self.radius * self.radius;

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let outward_normal = (r.at(root) - self.center(r.tm)) / self.radius;
        let (u, v) = self.get_sphere_uv(outward_normal);
        let mut rec = HitRecord::new(r.at(root), outward_normal, root, u, v, false, &self.mat_ptr);

        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let box0 = AABB::new(
            self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = AABB::new(
            self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(AABB::surrounding_box(box0, box1))
    }
}
