use crate::screen::ScreenBuffers;
use crate::object::Object;
use crate::camera::Camera;

pub struct Scene {
    pub screen: ScreenBuffers,
    pub objects: Vec<Object>,
    pub camera: Camera,
}
