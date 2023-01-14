use bevy::{prelude::*, render::mesh::VertexAttributeValues, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::{
    enemy::spawn_static_enemy, states::GameState, util::despawn_with, InGameItem, TextureHandles,
};
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            // ingame transitions
            .add_enter_system(GameState::InGame, setup_level)
            .add_exit_system(GameState::InGame, despawn_level);
    }
}

fn setup_level(
    mut commands: Commands,
    texture_handles: Res<TextureHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    images: ResMut<Assets<Image>>,
) {
    if let Some(cl) = images.get(&texture_handles.chalk_line_horizontal) {
        println!("{:?}", cl.sampler_descriptor)
    }

    const LEN: f32 = 10.;

    let mut mesh = Mesh::from(shape::Quad::new(Vec2::new(LEN, 2.5)));

    if let Some(VertexAttributeValues::Float32x2(uvs)) = mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0) {
        for uv in uvs {
            uv[0] *= 16. * LEN / 10.; // WHY DOES THIS WORK??????
            uv[1] *= 4.;
        }
    }

    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(5.0, 5.0))
        .insert(InGameItem)
        .insert(VisibilityBundle {
            visibility: Visibility::VISIBLE,
            ..default()
        })
        .insert(TransformBundle::from_transform(Transform::from_xyz(
            0.0, -9.0, 10.0,
        )))
        .with_children(|cb| {
            cb.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(mesh.into()).into(),
                visibility: Visibility::VISIBLE,
                transform: Transform::from_xyz(0.0, 4.0, 0.0),
                material: materials.add(ColorMaterial {
                    texture: Some(texture_handles.chalk_line_horizontal.clone()),
                    ..default()
                }),
                ..default()
            });
        });

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(
            100.0, 400.0, 0.0,
        )))
        .insert(InGameItem);
    // enemy
    spawn_static_enemy(&mut commands, &texture_handles, Vec3::new(10.0, 0.0, 10.0));
    spawn_static_enemy(&mut commands, &texture_handles, Vec3::new(5.0, 0.0, 10.0));
}

fn despawn_level(commands: Commands, query: Query<Entity, With<InGameItem>>) {
    despawn_with(commands, query)
}
