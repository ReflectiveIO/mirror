use super::Scene;
use crate::rays::geometry::Vector;
use crate::slg::cameras::{Camera as slgCameraTrait, CameraType as slgCameraType};
use crate::slg::EditAction::CameraEdit;

/// Camera stores camera definition.

/// Types of cameras.
pub enum CameraType {
    // This list must be aligned with slg::cameras::CameraType
    PERSPECTIVE,
    ORTHOGRAPHIC,
    STEREO,
    ENVIRONMENT,
}

impl From<&slgCameraType> for CameraType {
    fn from(t: &slgCameraType) -> Self {
        match t {
            slgCameraType::Perspective => CameraType::PERSPECTIVE,
            slgCameraType::Orthographic => CameraType::ORTHOGRAPHIC,
            slgCameraType::Stereo => CameraType::STEREO,
            slgCameraType::Environment => CameraType::ENVIRONMENT,
        }
    }
}

pub trait CameraTrait {
    /// Returns the camera type.
    fn get_type(&self) -> CameraType;

    /// Translates by vector t. This method can be used only
    /// when the Scene is not in use by a Session.
    fn translate(&mut self, x: f32, y: f32, z: f32);

    /// Translate left by t. This method can be used only
    /// when the Scene is not in use by a Session.
    fn translate_left(&mut self, t: f32);

    /// Translate right by t. This method can be used only
    /// when the Scene is not in use by a Session.
    fn translate_right(&mut self, t: f32);

    /// Translates forward by t. This method can be used only
    /// when the Scene is not in use by a Session.
    fn translate_forward(&mut self, t: f32);

    /// Translates backward by t. This method can be used only
    /// when Scene is not in use by a Session.
    fn translate_backward(&mut self, t: f32);

    /// Rotates by angle around the axis. This method can be used only
    /// when the Scene is not in use by Session.
    fn rotate(&mut self, angle: f32, x: f32, y: f32, z: f32);

    /// Rotates left by angle. This method can be used only when
    /// the Scene is not in use by Session.
    fn rotate_left(&mut self, angle: f32);

    /// Rotates right by angle. This method can be used only when
    /// the Scene is not in use by Session.
    fn rotate_right(&mut self, angle: f32);

    /// Rotates up by angle. This method can be used only when
    /// the Scene is not in use by Session.
    fn rotate_up(&mut self, angle: f32);

    /// Rotates down by angle. This method can be used only
    /// when the Scene is not in use by Session.
    fn rotate_down(&mut self, angle: f32);
}

pub struct Camera {
    scene: Scene,
}

impl Camera {
    pub fn new(scene: Scene) -> Self { Camera { scene } }
}

impl CameraTrait for Camera {
    fn get_type(&self) -> CameraType {
        CameraType::from(self.scene.scene.camera.unwrap().get_type())
    }

    fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.scene
            .scene
            .camera
            .unwrap()
            .translate(&Vector::new(x, y, z));
        self.scene.scene.edit_actions.add_action(CameraEdit);
    }

    fn translate_left(&mut self, t: f32) {
        self.scene.scene.camera.unwrap().translate_left(t);
        self.scene.scene.edit_actions.add_action(CameraEdit);
    }

    fn translate_right(&mut self, t: f32) {
        self.scene.scene.camera.unwrap().translate_right(t);
        self.scene.scene.edit_actions.add_action(CameraEdit);
    }

    fn translate_forward(&mut self, t: f32) {
        self.scene.scene.camera.unwrap().translate_forward(t);
        self.scene.scene.edit_actions.add_action(CameraEdit);
    }

    fn translate_backward(&mut self, t: f32) {
        self.scene.scene.camera.unwrap().translate_backward(t);
        self.scene.scene.edit_actions.add_action(CameraEdit);
    }

    fn rotate(&mut self, angle: f32, x: f32, y: f32, z: f32) {
        self.scene
            .scene
            .camera
            .unwrap()
            .rotate(angle, &Vector::new(x, y, z));
        self.scene.scene.edit_actions.add_action(CameraEdit);
    }

    fn rotate_left(&mut self, angle: f32) {
        self.scene.scene.camera.unwrap().rotate_left(angle);
        self.scene.scene.edit_actions.add_action(CameraEdit);
    }

    fn rotate_right(&mut self, angle: f32) {
        self.scene.scene.camera.unwrap().rotate_right(angle);
        self.scene.scene.edit_actions.add_action(CameraEdit);
    }

    fn rotate_up(&mut self, angle: f32) {
        self.scene.scene.camera.unwrap().rotate_up(angle);
        self.scene.scene.edit_actions.add_action(CameraEdit);
    }

    fn rotate_down(&mut self, angle: f32) {
        self.scene.scene.camera.unwrap().rotate_down(angle);
        self.scene.scene.edit_actions.add_action(CameraEdit);
    }
}
