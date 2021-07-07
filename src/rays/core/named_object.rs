use uuid::Uuid;

use crate::rays::Properties;

pub trait NamedObject {
    fn get_name(&self) -> &String;
    fn set_name(&mut self, name: &str);
}
