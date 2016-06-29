use std::fmt::Debug;
use Float;
use types::{Point, Vector, EPSILON_X, EPSILON_Y, EPSILON_Z};

use cgmath::InnerSpace;

mod bounding_box;

mod transformer;
pub use self::transformer::AffineTransformer;

mod boolean;
pub use self::boolean::{Union, Intersection};

mod sphere;
pub use self::sphere::Sphere;

mod cylinder;
pub use self::cylinder::{Cone, Cylinder};

mod slab;
pub use self::slab::{SlabX, SlabY, SlabZ};

pub const ALWAYS_PRECISE: Float = 1.;


pub fn normal_from_object(f: &Object, p: Point) -> Vector {
    let center = f.approx_value(p, ALWAYS_PRECISE);
    let dx = f.approx_value(p + EPSILON_X, ALWAYS_PRECISE) - center;
    let dy = f.approx_value(p + EPSILON_Y, ALWAYS_PRECISE) - center;
    let dz = f.approx_value(p + EPSILON_Z, ALWAYS_PRECISE) - center;
    Vector::new(dx, dy, dz).normalize()
}

pub trait Object: ObjectClone + Debug {
    fn bbox(&self) -> &bounding_box::BoundingBox {
        &bounding_box::INFINITY_BOX
    }
    // Value is 0 on object surfaces, negative inside and positive outside of objects.
    // If positive, value is guarateed to be the minimum distance to the object surface.
    // return some approximation (which is always larger then the real value).
    // Only do a proper calculation, for values smaller then precision.
    fn approx_value(&self, _: Point, _: Float) -> Float {
        unimplemented!();
    }
    fn normal(&self, _: Point) -> Vector {
        unimplemented!();
    }
    fn translate(&self, v: Vector) -> Box<Object> {
        AffineTransformer::new_translate(self.clone_box(), v)
    }
    fn rotate(&self, r: Vector) -> Box<Object> {
        AffineTransformer::new_rotate(self.clone_box(), r)
    }
    fn scale(&self, s: Vector) -> Box<Object> {
        AffineTransformer::new_scale(self.clone_box(), s)
    }
}

pub trait ObjectClone {
    fn clone_box(&self) -> Box<Object>;
}

impl<T> ObjectClone for T
    where T: 'static + Object + Clone
{
    fn clone_box(&self) -> Box<Object> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<Object> {
    fn clone(&self) -> Box<Object> {
        self.clone_box()
    }
}

// Objects never equal each other
impl PartialEq for Box<Object> {
    fn eq(&self, _: &Box<Object>) -> bool {
        false
    }
}

// Objects are never ordered
impl PartialOrd for Box<Object> {
    fn partial_cmp(&self, _: &Box<Object>) -> Option<::std::cmp::Ordering> {
        None
    }
}