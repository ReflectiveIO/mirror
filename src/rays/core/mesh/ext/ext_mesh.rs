use downcast_rs::Downcast;

use crate::rays::mesh::Mesh;

pub trait ExtMesh: Mesh + Downcast {}
impl_downcast!(ExtMesh);
