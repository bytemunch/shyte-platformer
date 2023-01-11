use bevy::prelude::*;
use bevy_parallax::{LayerData, ParallaxPlugin, ParallaxResource};

const BG_SCALE: f32 = 1. / 20.; // magic number (:

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ParallaxResource {
            layer_data: vec![LayerData {
                speed: 0.,
                path: "img/bg.png".to_string(),
                tile_size: Vec2::new(9884., 2808.),
                cols: 1,
                rows: 1,
                scale: BG_SCALE,
                z: 0.0,
                position: Vec2::new(0., -20.),
                ..Default::default()
            }],
            ..Default::default()
        })
        .add_plugin(ParallaxPlugin);
    }
}
