pub type UV = na::Vector2<f32>;

mod test {
    use crate::rays::geometry::UV;

    #[test]
    fn test_uv_constructor() {
        let uv1: UV = UV::new(0.1, 0.1);
        let uv2: UV = UV::new(0.1, 0.1);

        let uv3: UV = uv1 * uv2;
    }
}
