use bevy::{
    input::Input,
    prelude::{KeyCode, OrthographicProjection, Query, Res, With},
    render::camera::Camera,
    time::Time,
};

const CAMERA_SPEED_PER_SEC: f32 = 3.0;

// handle zoom
pub fn camera_control(
    kb: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    let dist = CAMERA_SPEED_PER_SEC * time.delta().as_secs_f32();

    for mut projection in query.iter_mut() {
        let mut log_scale = projection.scale.ln();

        if kb.pressed(KeyCode::PageUp) {
            log_scale -= dist;
        } else if kb.pressed(KeyCode::PageDown) {
            log_scale += dist;
        }

        projection.scale = log_scale.exp();
    }
}
