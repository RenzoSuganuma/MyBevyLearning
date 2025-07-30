mod bevy_2d_bloom;

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
        .add_systems(Startup, bevy_2d_bloom::setup)
        .add_systems(Update, bevy_2d_bloom::update_bloom_settings)
        .run();
}
