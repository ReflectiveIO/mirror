use config::*;

#[test]
fn test_load_scene() {
    let mut props = Config::default();
    props
        .merge(File::with_name("scenes/alloy/scene.toml"))
        .unwrap();

    assert_eq!(
        props.get("scene.objects.mat_alloy.ply").ok(),
        Some("scenes/alloy/mat_alloy.ply".to_string())
    );

    let arr: Vec<f64> = props.get("scene.materials.mat1.kd").unwrap();
    assert_eq!(arr, vec![0.5, 0.5, 0.5]);
}
