pub enum EditAction {
    // Use this for any Camera parameter editing
    CameraEdit = 1 << 0,
    // Use this for any DataSet related editing
    GeometryEdit = 1 << 1,
    // Use this for any instance transformation related editing
    GeometryTransEdit = 1 << 2,
    // Use this for any Material related editing
    MaterialsEdit = 1 << 3,
    // Use this if the kind of materials used changes
    MaterialTypesEdit = 1 << 4,
    // Use this for any Light related editing
    LightsEdit = 1 << 5,
    // Use this if the kind of lights used changes
    LightTypesEdit = 1 << 6,
    // Use this for any ImageMaps related editing
    ImageMapsEdit = 1 << 7,
}

pub struct EditActionList {
    actions: u32,
}

impl EditActionList {
    pub fn new() -> Self {
        EditActionList { actions: 0 }
    }

    pub fn add_action(&mut self, a: EditAction) {
        self.actions |= a as u32;
    }
}
