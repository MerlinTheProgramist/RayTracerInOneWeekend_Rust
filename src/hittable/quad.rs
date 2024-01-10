#![allow(non_snake_case)]

use crate::{
    aabb::AABB,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{cross, dot, Point3, Vec3},
    Num,
};

use super::{HitRecord, Hittable, HittableObject};

pub struct Quad {
    Q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Box<Material>,
    bbox: AABB,
    n: Vec3,
    D: Num,
    w: Vec3,
}

impl Quad {
    pub fn new(Q: Point3, u: Vec3, v: Vec3, mat: Box<Material>) -> HittableObject {
        // we can get the surface normal with cross product of `u` and `v`
        // The plane is defined as all points (x,y,z) that satisfy that equation.
        // We know that Q lies on the plane, so that's enought to solve for D
        // D = n.x*Q.x + n.y*Q.y + n.z*Q.z = n · Q
        let n = cross(&u, &v);
        HittableObject::Quad(Self {
            Q,
            u,
            v,
            mat,
            bbox: AABB::from_corners(Q, Q + u + v).padded(),
            n,
            D: dot(&n, &Q),
            w: n / dot(&n, &n),
        })
    }
}

// general formula for a point on plane
// Ax + By + Cz + D = 0
// D_ = Ax + By + Cz // unknown D_
// D_ = dot(n,v) // where: n=normal, v=direction of ray R(t)=P+td
// D_ = dot(n, (P+td))
// solving for t:
// dot(n, P) + dot(n, td) = D_
// dot(n, P) + t*dot(n, d) = D_
// t = (D-n·P)/(n·d)
// plugging to that `t` ray equatio gives intersection point
// when the denominator (n·d) == 0, then no intersection occured
// also when t is less than the minimum acceptable value, we also record a miss

// Then we need to determinate if the point is inside our parallelogram:
///// Firstly translate the intersection point to planar 2d coordinates on the plane
// Let P = point of intersection of the Ray with our plane
// Let Q = corner from which u and v orininate
// Let u and v be the vectors along edges  of our parallelogram

// We want to find a and b, the scale of vectors where the Ray hit
//     P = Q + au + bv

// Let p be the vector from Q to P
//    p = P - Q = au + bv

// Cross above with u and v respecrively
//    u ⨯ p = u ⨯ (au + bv)
//          = u ⨯ au + u ⨯ bv
//          = a(u ⨯ u) + b(u ⨯ v)
//    u ⨯ p = b(u ⨯ v)

//    v ⨯ p = v ⨯ (au + bv)
//          = v ⨯ au + v ⨯ bv
//          = a(v ⨯ u) + b(v ⨯ v)
//    v ⨯ p = a(v ⨯ u)

// To solve for a and b, we need to divide by (v ⨯ u) but since those are vectors,
// we firstly apply the dot product with the normal of the plane n = (v⨯u), to both sides, resolving in a number, so we can then divide.
//    a = (n · (v ⨯ p)) / (n · (v ⨯ u))
//    b = (n · (u ⨯ p)) / (n · (u ⨯ v)) // reverse coefficients of top and bottom cross products (a⨯b=-b⨯a)
//      = (n · (p ⨯ v)) / (n · (v ⨯ u))

// We can simplify by precalculating a vector w that will be contsnt for the plane's baisis frame, for any planar point P ahead
//    w = n / (n·(u⨯v)) = n / (n·n)
//    a = w · (p ⨯ v)
//    b = w · (u ⨯ p)

///// Then test if its inside the parallelogram:
// 0 <= a <= 1
// 0 <= b <= 1

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<super::HitRecord> {
        let denom = dot(&self.n, r.direction());

        // No hit if the ray is parallel to the plane
        if denom.abs() < 1e-8 {
            return None;
        }

        // Return None if the hit point parameter t is outside the ray interval
        let t = (self.D - dot(&self.n, r.origin())) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.Q;
        let alpha = dot(&self.w, &cross(&planar_hitpt_vector, &self.v));
        let beta = dot(&self.w, &cross(&self.u, &planar_hitpt_vector));

        if !is_interiour(alpha, beta) {
            return None;
        }

        Some(HitRecord::new(
            intersection,
            t,
            self.mat.clone(),
            alpha,
            beta,
        ))
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

fn is_interiour(a: Num, b: Num) -> bool {
    0.0 < a && a < 1.0 && 0.0 < b && b < 1.0
}
