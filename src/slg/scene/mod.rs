use crate::rays::Dataset;
use crate::slg::cameras::{Camera, CameraTrait, CameraType, EnvironmentCamera};
use crate::slg::{EditAction, EditActionList};

pub struct Scene {
    pub camera: Box<dyn CameraTrait>,
    pub edit_actions: EditActionList,
    pub dataset: Dataset,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            camera: Box::new(EnvironmentCamera::new()),
            edit_actions: EditActionList::new(),
            dataset: Dataset::new(None),
        }
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
