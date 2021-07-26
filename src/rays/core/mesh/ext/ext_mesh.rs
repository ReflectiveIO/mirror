use downcast_rs::Downcast;

use crate::rays::mesh::Mesh;
use crate::rays::object::NamedObject;

pub trait ExtMesh: Mesh + Downcast {}
impl_downcast!(ExtMesh);
