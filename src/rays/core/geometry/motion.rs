use crate::rays::geometry::{Matrix4x4, Transform};
use crate::rays::Properties;

#[derive(Clone)]
pub struct InterpolatedTransform {
    pub start_time: f32,
    pub end_time: f32,
    pub start: Transform,
    pub end: Transform,
}

#[derive(Default, Clone)]
pub struct MotionSystem {
    times: Vec<f32>,
    interpolated_transforms: Vec<InterpolatedTransform>,
    interpolated_inverse_transforms: Vec<InterpolatedTransform>,
}

impl MotionSystem {
    pub fn is_static(&self) -> bool { todo!() }

    pub fn start_time(&self) -> Option<&f32> { self.times.first() }

    pub fn end_time(&self) -> Option<&f32> { self.times.last() }

    pub fn sample(&self, time: f32) -> Matrix4x4 { todo!() }

    pub fn sample_inverse(&self, time: f32) -> Matrix4x4 { todo!() }

    pub fn to_properties(&self, prefix: &str, storing_global2local: bool) -> Properties {
        todo!()
        // let mut props = Properties::new();
        //
        // // First and last interpolated_transforms have the same
        // transformation start and // end
        // for i in 1..self.interpolated_transforms.len() - 1 {
        //     let it = self.interpolated_transforms.get(i).unwrap();
        //
        //     props.set(
        //         format!("{}.motion.{}.time", prefix, i - 1),
        //         it.start_time as f64,
        //     );
        //
        //     let m = match storing_global2local {
        //         true => it.start.matrix().try_inverse().unwrap().iter(),
        //         false => it.start.matrix().iter(),
        //     };
        //     props.set(
        //         format!("{}.motion.{}.transformation", prefix, i - 1),
        //         m.map(|&n| n as f64).collect::<Vec<f64>>(),
        //     )
        // }
        //
        // let last_index = self.interpolated_transforms.len() - 2;
        // let it = self.interpolated_transforms.get(last_index).unwrap();
        // props.set(
        //     format!("{}.motion.{}.time", prefix, last_index),
        //     it.end_time as f64,
        // );
        //
        // let m = match storing_global2local {
        //     true => it.end.matrix().try_inverse().unwrap().iter(),
        //     false => it.end.matrix().iter(),
        // };
        // props.set(
        //     format!("{}.motion.{}.transformation", prefix, last_index),
        //     m.map(|&n| n as f64).collect::<Vec<f64>>(),
        // );
        //
        // return props;
    }
}
