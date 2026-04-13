use super::*;

#[test]
fn renders_operations_surface_target() {
    let markup = OperationsSurface::builder().build().render().into_string();

    assert!(markup.contains("id=\"operations-surface\""));
    assert!(markup.contains("data-operations-surface"));
    assert!(markup.contains("network-log-target"));
}
