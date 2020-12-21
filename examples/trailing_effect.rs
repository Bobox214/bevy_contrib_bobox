use bevy::{
    prelude::*,
    render::camera::{Camera, OrthographicProjection},
};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(velocity_system.system())
        .run();
}
struct Velocity(Vec2);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("icon.png");
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            sprite: Sprite {
                size: Vec2::new(50.0, 50.0),
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(Velocity(Vec2::new(100.0, 100.0)));
}

fn velocity_system(
    time: Res<Time>,
    cameras: Query<(&Camera, &OrthographicProjection)>,
    mut query: Query<(Mut<Transform>, Mut<Velocity>)>,
) {
    let mut projection = None;
    for (camera, camera_projection) in cameras.iter() {
        if camera.name == Some("Camera2d".to_string()) {
            projection = Some(camera_projection);
            break;
        }
    }
    if let Some(projection) = projection {
        for (mut transform, mut velocity) in query.iter_mut() {
            *transform.translation.x_mut() += velocity.0.x() * time.delta_seconds;
            *transform.translation.y_mut() += velocity.0.y() * time.delta_seconds;
            if (transform.translation.x() < projection.left && velocity.0.x() < 0.0)
                || (transform.translation.x() > projection.right && velocity.0.x() > 0.0)
            {
                *velocity.0.x_mut() = -velocity.0.x();
            }
            if (transform.translation.y() < projection.bottom && velocity.0.y() < 0.0)
                || (transform.translation.y() > projection.top && velocity.0.y() > 0.0)
            {
                *velocity.0.y_mut() = -velocity.0.y();
            }
        }
    }
}
