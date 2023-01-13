use bevy::prelude::*;
use bevy_parallax::{LayerData, ParallaxPlugin, ParallaxResource};

use crate::CAMERA_SCALE;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ParallaxResource {
            layer_data: vec![LayerData {
                speed: 0.,
                path: "img/bg.png".to_string(),
                tile_size: Vec2::new(4992., 1404.),
                cols: 1,
                rows: 1,
                scale: CAMERA_SCALE / 1.8,
                z: 0.0,
                position: Vec2::new(0., 0.),
                ..Default::default()
            }],
            ..Default::default()
        })
        .add_plugin(ParallaxPlugin);
    }
}
