use crate::point::Point;
use crate::ray::Ray;
use crate::transformation::Transformation;
use crate::vector::Vector;

trait FireRay {
    fn fire_ray(&self, u: f32, v: f32) -> Ray;
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct OrthogonalCamera {
    aspect_ratio: f32,
    tranformation: Transformation,
}

impl OrthogonalCamera {
    pub fn new(aspect_ratio: f32, tranformation: Transformation) -> OrthogonalCamera {
        OrthogonalCamera {
            aspect_ratio,
            tranformation,
        }
    }
}

impl FireRay for OrthogonalCamera {
    fn fire_ray(&self, u: f32, v: f32) -> Ray {
        self.tranformation
            * Ray {
                origin: Point::from((-1.0, (1.0 - 2.0 * u) * self.aspect_ratio, 2.0 * v - 1.0)),
                dir: Vector::from((1.0, 0.0, 0.0)),
                ..Default::default()
            }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct PerspectiveCamera {
    distance: f32,
    aspect_ratio: f32,
    transformation: Transformation,
}

impl PerspectiveCamera {
    pub fn new(
        aspect_ratio: f32,
        distance: f32,
        transformation: Transformation,
    ) -> PerspectiveCamera {
        PerspectiveCamera {
            aspect_ratio,
            distance,
            transformation,
        }
    }
}

impl FireRay for PerspectiveCamera {
    fn fire_ray(&self, u: f32, v: f32) -> Ray {
        self.transformation
            * Ray {
                origin: Point::from((-self.distance, 0.0, 0.0)),
                dir: Vector::from((
                    self.distance,
                    (1.0 - 2.0 * u) * self.aspect_ratio,
                    2.0 * v - 1.0,
                )),
                ..Default::default()
            }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::misc::IsClose;

    #[test]
    fn test_orthogonal_camera() {
        let cam = OrthogonalCamera {
            aspect_ratio: 2.0,
            ..Default::default()
        };
        let ray1 = cam.fire_ray(0.0, 0.0);
        let ray2 = cam.fire_ray(1.0, 0.0);
        let ray3 = cam.fire_ray(0.0, 1.0);
        let ray4 = cam.fire_ray(1.0, 1.0);

        assert!((ray1.dir * ray2.dir).squared_norm().is_close(0.0));
        assert!((ray1.dir * ray3.dir).squared_norm().is_close(0.0));
        assert!((ray1.dir * ray4.dir).squared_norm().is_close(0.0));

        assert!(ray1.at(1.0).is_close(Point::from((0.0, 2.0, -1.0))));
        assert!(ray2.at(1.0).is_close(Point::from((0.0, -2.0, -1.0))));
        assert!(ray3.at(1.0).is_close(Point::from((0.0, 2.0, 1.0))));
        assert!(ray4.at(1.0).is_close(Point::from((0.0, -2.0, 1.0))));
    }

    #[test]
    fn test_perspective_camera() {
        let cam = PerspectiveCamera {
            distance: 1.0,
            aspect_ratio: 2.0,
            ..Default::default()
        };
        let ray1 = cam.fire_ray(0.0, 0.0);
        let ray2 = cam.fire_ray(1.0, 0.0);
        let ray3 = cam.fire_ray(0.0, 1.0);
        let ray4 = cam.fire_ray(1.0, 1.0);

        assert!(ray1.origin.is_close(ray2.origin));
        assert!(ray1.origin.is_close(ray3.origin));
        assert!(ray1.origin.is_close(ray4.origin));

        assert!(ray1.at(1.0).is_close(Point::from((0.0, 2.0, -1.0))));
        assert!(ray2.at(1.0).is_close(Point::from((0.0, -2.0, -1.0))));
        assert!(ray3.at(1.0).is_close(Point::from((0.0, 2.0, 1.0))));
        assert!(ray4.at(1.0).is_close(Point::from((0.0, -2.0, 1.0))));
    }
}
