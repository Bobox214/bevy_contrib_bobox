use bevy::prelude::*;
use bevy_contrib_bobox::{Outline2dPlugin, OutlineConfiguration, OutlineMaterial};
fn main() {
    //env_logger::init();
    App::build()
        .add_resource(WindowDescriptor {
            width: 600,
            height: 400,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(Outline2dPlugin)
        .add_startup_system(setup.system())
        .add_system(input_system.system())
        .add_system(update_system.system())
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
                translation: Vec3::new(-100.0, -20.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(outline_materials.add(OutlineMaterial {
            configuration: OutlineConfiguration {
                color: Color::rgba(1.0, 0.0, 0.0, 1.0),
                ..Default::default()
            },
            with_outline: true,
        }));

    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: color_materials.add(asset_server.load("playerShip1_red.png").into()),
            transform: Transform {
                translation: Vec3::new(150.0, -20.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(outline_materials.add(OutlineMaterial {
            configuration: OutlineConfiguration {
                color: Color::rgba(1.0, 0.5, 1.0, 1.0),
                ..Default::default()
            },
            with_outline: true,
        }));
    commands
        .spawn(UiCameraComponents::default())
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "".to_string(),
                font: asset_server.load("FiraSans-Bold.ttf"),
                style: TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            },
            ..Default::default()
        });
    // Just to ease keyboard input modification and text visualization
    commands.insert_resource(OutlineMaterial {
        with_outline: true,
        configuration: OutlineConfiguration {
            width: 3,
            inside: 1,
            ..Default::default()
        },
    });
}

fn input_system(input: Res<Input<KeyCode>>, mut ref_material: ResMut<OutlineMaterial>) {
    if input.just_pressed(KeyCode::Space) {
        ref_material.with_outline = !ref_material.with_outline;
    }
    if input.just_pressed(KeyCode::I) {
        ref_material.configuration.inside = if ref_material.configuration.inside == 0 {
            1
        } else {
            0
        };
    }
    if input.just_pressed(KeyCode::W) {
        ref_material.configuration.width = match ref_material.configuration.width {
            x if x < 10 => x + 1,
            _ => 1,
        };
    }
}
fn update_system(
    ref_material: ChangedRes<OutlineMaterial>,
    mut outline_materials: ResMut<Assets<OutlineMaterial>>,
    query: Query<&Handle<OutlineMaterial>>,
    mut texts: Query<Mut<Text>>,
) {
    for handle in query.iter() {
        let mut outline_material = outline_materials.get_mut(handle).unwrap();
        outline_material.with_outline = ref_material.with_outline;
        outline_material.configuration.width = ref_material.configuration.width;
        outline_material.configuration.inside = ref_material.configuration.inside;
    }
    for mut text in texts.iter_mut() {
        text.value = format!(
            "Use keys <Space> <I> <W>\nwith_outline: {}\nwidth: {}\ninside: {}",
            ref_material.with_outline,
            ref_material.configuration.width,
            ref_material.configuration.inside
        );
    }
}
