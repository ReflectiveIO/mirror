use std::ops::{BitAnd, BitOr};

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

impl BitAnd<EditAction> for u32 {
    type Output = u32;

    fn bitand(self, rhs: EditAction) -> Self::Output {
        self & rhs as u32
    }
}

impl BitOr<EditAction> for EditAction {
    type Output = u32;

    fn bitor(self, rhs: EditAction) -> Self::Output {
        self as u32 | rhs as u32
    }
}

impl PartialEq<EditAction> for u32 {
    fn eq(&self, other: &EditAction) -> bool {
        self == other
    }

    fn ne(&self, other: &EditAction) -> bool {
        self != other
    }
}

#[derive(Clone, Debug)]
pub struct EditActionList {
    actions: u32,
}

impl EditActionList {
    pub fn new() -> Self {
        EditActionList { actions: 0 }
    }

    pub fn reset(&mut self) {
        self.actions = 0;
    }

    pub fn add_action(&mut self, a: EditAction) {
        self.actions |= a as u32;
    }

    pub fn add_all_action(&mut self) {
        self.add_action(EditAction::CameraEdit);
        self.add_action(EditAction::GeometryEdit);
        self.add_action(EditAction::GeometryTransEdit);
        self.add_action(EditAction::MaterialsEdit);
        self.add_action(EditAction::MaterialTypesEdit);
        self.add_action(EditAction::LightsEdit);
        self.add_action(EditAction::LightTypesEdit);
        self.add_action(EditAction::ImageMapsEdit);
    }

    pub fn add_actions(&mut self, a: u32) {
        self.actions |= a;
    }

    pub fn get_actions(&self) -> u32 {
        self.actions
    }

    pub fn has(&self, a: EditAction) -> bool {
        (self.actions & a) != 0
    }

    pub fn has_only(&self, a: EditAction) -> bool {
        self.actions == a
    }

    pub fn has_any_action(&self) -> bool {
        self.actions != 0
    }
}
