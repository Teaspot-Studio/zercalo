use super::camera::Camera;
use super::Scene;
use glam::Vec3;

/// Trait for complex scenes that has embedded scene inside
pub trait HasScene {
    fn get_scene(&self) -> &Scene;
}

impl HasScene for Scene {
    fn get_scene(&self) -> &Scene {
        self
    }
}

/// Trait for complex scenes that has embedded mutable scene inside
pub trait HasMutScene {
    fn get_scene_mut(&mut self) -> &mut Scene;
}

impl HasMutScene for Scene {
    fn get_scene_mut(&mut self) -> &mut Scene {
        self
    }
}

/// Trait that allows to access substate with camera
pub trait HasCamera {
    fn get_camera(&self) -> &Camera;
}

impl HasCamera for Scene {
    fn get_camera(&self) -> &Camera {
        &self.camera
    }
}

/// Trait that allows to access substate with camera
pub trait HasMutCamera {
    fn get_mut_camera(&mut self) -> &mut Camera;
}

impl HasMutCamera for Scene {
    fn get_mut_camera(&mut self) -> &mut Camera {
        &mut self.camera
    }
}

/// Trait that allows to access bounding volume of inner scene
pub trait HasBounding {
    fn get_bounding_volume(&self) -> (Vec3, Vec3);

    fn get_bounding_center(&self) -> Vec3 {
        let (minv, maxv) = self.get_bounding_volume();
        (maxv - minv) * 0.5
    }
}

impl HasBounding for Scene {
    fn get_bounding_volume(&self) -> (Vec3, Vec3) {
        self.bounding()
    }

    fn get_bounding_center(&self) -> Vec3 {
        self.center()
    }
}
