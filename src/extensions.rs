use bevy::prelude::{Camera, GlobalTransform, Mat4, Vec3};

pub trait Trafo {
    fn ndc_to_world(&self, camera_transform: &GlobalTransform, ndc: Vec3) -> Option<Vec3>;
}

impl Trafo for Camera {
    /// Convert from normalized device coordinates (range -1 to 1 on all axes) to world position.
    ///
    /// Returns `None` if the projection fails for any reason. Otherwise the projected vector is returned
    fn ndc_to_world(&self, camera_transform: &GlobalTransform, ndc: Vec3) -> Option<Vec3> {
        let ndc_to_world: Mat4 =
            camera_transform.compute_matrix() * self.projection_matrix().inverse();
        let world_coords = ndc_to_world.project_point3(ndc);
        if !world_coords.is_nan() {
            Some(world_coords)
        } else {
            None
        }
    }
}
