use crate::na::storage::ContiguousStorage;
use crate::rays::geometry::Transform;
use crate::rays::Properties;

pub struct InterpolatedTransform {
    pub start_time: f32,
    pub end_time: f32,
    pub start: Transform,
    pub end: Transform,
}

#[derive(Default)]
pub struct MotionSystem {
    times: Vec<f32>,
    interpolated_transforms: Vec<InterpolatedTransform>,
    interpolated_inverse_transforms: Vec<InterpolatedTransform>,
}

impl MotionSystem {
    pub fn to_properties(&self, prefix: &str, storing_global2local: bool) -> Properties {
        let mut props = Properties::new();

        // First and last interpolated_transforms have the same transformation start and
        // end
        for i in 1..self.interpolated_transforms.len() - 1 {
            let it = self.interpolated_transforms.get(i).unwrap();

            props.set(
                format!("{}.motion.{}.time", prefix, i - 1),
                it.start_time as f64,
            );

            let m = match storing_global2local {
                true => it.start.matrix().try_inverse().unwrap().iter(),
                false => it.start.matrix().iter(),
            };
            props.set(
                format!("{}.motion.{}.transformation", prefix, i - 1),
                m.map(|&n| n as f64).collect::<Vec<f64>>(),
            )
        }

        let last_index = self.interpolated_transforms.len() - 2;
        let it = self.interpolated_transforms.get(last_index).unwrap();
        props.set(
            format!("{}.motion.{}.time", prefix, last_index),
            it.end_time as f64,
        );

        let m = match storing_global2local {
            true => it.end.matrix().try_inverse().unwrap().iter(),
            false => it.end.matrix().iter(),
        };
        props.set(
            format!("{}.motion.{}.transformation", prefix, last_index),
            m.map(|&n| n as f64).collect::<Vec<f64>>(),
        );

        return props;
    }
}
