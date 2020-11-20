use bevy::prelude::*;
use bevy_contrib_bobox::{Outline2dPlugin, OutlineMaterial};
fn main() {
    //env_logger::init();
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(Outline2dPlugin)
        .add_startup_system(setup.system())
        .add_system(input_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut outline_materials: ResMut<Assets<OutlineMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: color_materials.add(asset_server.load("icon.png").into()),
            transform: Transform {
                translation: Vec3::new(-200.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(outline_materials.add(OutlineMaterial {
            color: Color::rgba(1.0, 0.0, 0.0, 1.0),
            with_outline: true,
        }));

    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: color_materials.add(asset_server.load("playerShip1_red.png").into()),
            transform: Transform {
                translation: Vec3::new(200.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(outline_materials.add(OutlineMaterial {
            color: Color::rgba(1.0, 0.5, 1.0, 1.0),
            with_outline: true,
        }));
}

fn input_system(
    input: Res<Input<KeyCode>>,
    mut outline_materials: ResMut<Assets<OutlineMaterial>>,
    query: Query<&Handle<OutlineMaterial>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for handle in query.iter() {
            let mut outline_material = outline_materials.get_mut(handle).unwrap();
            outline_material.with_outline = !outline_material.with_outline;
        }
    }
}
