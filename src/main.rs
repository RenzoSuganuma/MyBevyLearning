use bevy::{
    core_pipeline::{
        bloom::{Bloom, BloomCompositeMode},
        tonemapping::{DebandDither, Tonemapping},
    },
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, &self::setup)
        .add_systems(Update, &self::update_bloom_settings)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true,
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Tonemapping::TonyMcMapface,
        Bloom::default(),
        DebandDither::Enabled,
    ));

    // sprite
    commands.spawn(Sprite {
        image: asset_server.load("branding/bevy_bird_dark.png"),
        color: Color::srgb(5.0, 5.0, 5.0),
        custom_size: Some(Vec2::splat(160.0)),
        ..default()
    });

    // circle mesh
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(100.))),
        MeshMaterial2d(materials.add(Color::srgb(7.5, 0.0, 7.5))),
        Transform::from_translation(Vec3::new(-200.0, 0.0, 0.0)),
    ));

    // hexagon mesh
    commands.spawn((
        Mesh2d(meshes.add(RegularPolygon::new(100., 6))),
        MeshMaterial2d(materials.add(Color::srgb(6.25, 9.4, 9.1))),
        Transform::from_translation(Vec3::new(200., 0., 0.0)),
    ));

    // ui
    commands.spawn((
        Text::default(),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

// ---

fn update_bloom_settings(
    camera: Single<(Entity, &Tonemapping, Option<&mut Bloom>), With<Camera>>,
    mut text: Single<&mut Text>,
    mut commands: Commands,
    keycode: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (camera_entity, tonemapping, bloom) = camera.into_inner();

    match bloom {
        Some(mut bloom) => {
            text.0 = "Bloom (Toggle: Space)\n".to_string();
            text.push_str(&format!("(Q/A) Intensity:{}\n", bloom.intensity));
            text.push_str(&format!(
                "(W/S) Low-frecuency boost:{}\n",
                bloom.low_frequency_boost
            ));
            text.push_str(&format!(
                "(E/D) Low-frecuency boost curvature:{}\n",
                bloom.low_frequency_boost_curvature
            ));
            text.push_str(&format!(
                "(R/F) High-pass frequency:{}\n",
                bloom.high_pass_frequency
            ));
            text.push_str(&format!(
                "(T/G) Mode:{}\n",
                match bloom.composite_mode {
                    BloomCompositeMode::EnergyConserving => "EnergyConserving",
                    BloomCompositeMode::Additive => "Additive",
                }
            ));
            text.push_str(&format!("(Y/H) Threshord:{}\n", bloom.prefilter.threshold));
            text.push_str(&format!(
                "(U/J) Threshold softness:{}\n",
                bloom.prefilter.threshold_softness
            ));
            text.push_str(&format!("(I/K) Horizontal Scale:{}\n", bloom.scale.x));

            if keycode.just_pressed(KeyCode::Space) {
                commands.entity(camera_entity).remove::<Bloom>();
            }

            let dt = time.delta_secs();

            if keycode.pressed(KeyCode::KeyA) {
                bloom.intensity -= dt / 10.0;
            }

            if keycode.pressed(KeyCode::KeyQ) {
                bloom.intensity += dt / 10.0;
            }

            bloom.intensity = bloom.intensity.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::KeyS) {
                bloom.low_frequency_boost -= dt / 10.0;
            }

            if keycode.pressed(KeyCode::KeyW) {
                bloom.low_frequency_boost += dt / 10.0;
            }

            bloom.low_frequency_boost = bloom.low_frequency_boost.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::KeyD) {
                bloom.low_frequency_boost_curvature -= dt / 10.0;
            }

            if keycode.pressed(KeyCode::KeyE) {
                bloom.low_frequency_boost_curvature += dt / 10.0;
            }

            bloom.low_frequency_boost_curvature =
                bloom.low_frequency_boost_curvature.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::KeyF) {
                bloom.high_pass_frequency -= dt / 10.0;
            }

            if keycode.pressed(KeyCode::KeyR) {
                bloom.high_pass_frequency += dt / 10.0;
            }

            bloom.high_pass_frequency = bloom.high_pass_frequency.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::KeyG) {
                bloom.composite_mode = BloomCompositeMode::Additive;
            }

            if keycode.pressed(KeyCode::KeyT) {
                bloom.composite_mode = BloomCompositeMode::EnergyConserving;
            }

            if keycode.pressed(KeyCode::KeyH) {
                bloom.prefilter.threshold -= dt / 10.0;
            }

            if keycode.pressed(KeyCode::KeyY) {
                bloom.prefilter.threshold += dt / 10.0;
            }

            bloom.prefilter.threshold = bloom.prefilter.threshold.max(0.0);

            if keycode.pressed(KeyCode::KeyJ) {
                bloom.prefilter.threshold_softness -= dt / 10.0;
            }

            if keycode.pressed(KeyCode::KeyU) {
                bloom.prefilter.threshold_softness += dt / 10.0;
            }

            bloom.prefilter.threshold_softness = bloom.prefilter.threshold_softness.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::KeyK) {
                bloom.scale.x -= dt * 2.0;
            }

            if keycode.pressed(KeyCode::KeyI) {
                bloom.scale.x += dt * 2.0;
            }

            bloom.scale.x = bloom.scale.x.clamp(0.0, 16.0);
        }

        None => {
            text.0 = "Bloom: Off(Toggle: Space)\n".to_string();

            if keycode.just_pressed(KeyCode::Space) {
                commands.entity(camera_entity).insert(Bloom::default());
            }
        }
    }

    text.push_str(&format!("(O) Tonemapping:{:?}\n", tonemapping));
    if keycode.just_pressed(KeyCode::KeyO) {
        commands
            .entity(camera_entity)
            .insert(next_tonemap(tonemapping));
    }
}

fn next_tonemap(tonemapping: &Tonemapping) -> Tonemapping {
    match tonemapping {
        Tonemapping::None => Tonemapping::AcesFitted,
        Tonemapping::AcesFitted => Tonemapping::AgX,
        Tonemapping::AgX => Tonemapping::BlenderFilmic,
        Tonemapping::BlenderFilmic => Tonemapping::Reinhard,
        Tonemapping::Reinhard => Tonemapping::ReinhardLuminance,
        Tonemapping::ReinhardLuminance => Tonemapping::SomewhatBoringDisplayTransform,
        Tonemapping::SomewhatBoringDisplayTransform => Tonemapping::TonyMcMapface,
        Tonemapping::TonyMcMapface => Tonemapping::None,
    }
}
