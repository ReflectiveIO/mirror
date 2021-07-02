use crate::rays::Properties;

pub trait NamedObject {
    fn get_name(&self) -> &String;
    fn set_name(&mut self, name: &str);

    /// Returns the Properties required to create this object
    fn to_properties(&self) -> Properties;

    fn get_unique_name(prefix: &str) -> String;
}
