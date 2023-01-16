use std::f32::consts::PI;

use bevy::{prelude::*, render::mesh::VertexAttributeValues, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionHelpers, IntoConditionalSystem};

use crate::{
    enemy::{spawn_enemy, EnemyMover},
    states::{GameState, PauseState},
    util::despawn_with,
    Actor, ActorDead, InGameItem, TextureHandles, DEATHPLANE,
};
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            // ingame transitions
            .add_enter_system(GameState::InGame, setup_level)
            .add_exit_system(GameState::InGame, despawn_level)
            .add_system(
                actor_fall_out
                    .run_in_state(GameState::InGame)
                    .run_in_state(PauseState::Running),
            );
    }
}

#[derive(Component)]
pub struct TagBox;

#[derive(Component)]
struct BoxTopLeft(Vec2);

#[derive(Component)]
struct BoxBottomRight(Vec2);

#[derive(Bundle)]
struct BoxBundle {
    tl: BoxTopLeft,
    br: BoxBottomRight,
    collider: Collider,
    transform_bundle: TransformBundle,

    _tag: TagBox,
    _igi: InGameItem,
    _vb: VisibilityBundle,
}

impl Default for BoxBundle {
    fn default() -> Self {
        Self {
            tl: BoxTopLeft(Vec2::new(-5.0, -5.0)),
            br: BoxBottomRight(Vec2::new(5.0, 5.0)),
            collider: Collider::cuboid(5.0, 5.0),
            transform_bundle: TransformBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),

            _tag: TagBox,
            _igi: InGameItem,
            _vb: VisibilityBundle::default(),
        }
    }
}

//helper
// TODO overload for Pos/Size style, to learn overloading
pub fn create_box(
    commands: &mut Commands,
    tl: Vec2,
    br: Vec2,
    texture_handles: &Res<TextureHandles>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    let w = (br.x - tl.x).abs();
    let h = (br.y - tl.y).abs();

    let hx = w / 2.;
    let hy = h / 2.;

    let cx = (br.x - tl.x) / 2.;
    let cy = (br.y - tl.y) / 2.;

    const UV_MAGIC_NUMBER: f32 = 32.;

    let mut x_mesh = Mesh::from(shape::Quad::new(Vec2::new(w, 2.5)));
    let mut y_mesh = Mesh::from(shape::Quad::new(Vec2::new(h, 2.5)));

    if let Some(VertexAttributeValues::Float32x2(uvs)) = x_mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0)
    {
        for uv in uvs {
            uv[0] *= 16. * w / UV_MAGIC_NUMBER; // WHY DOES THIS WORK??????
            uv[1] *= 4.;
        }
    }

    if let Some(VertexAttributeValues::Float32x2(uvs)) = y_mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0)
    {
        for uv in uvs {
            uv[1] *= 4.;
            uv[0] *= 16. * h / UV_MAGIC_NUMBER;
        }
    }

    commands
        .spawn(BoxBundle {
            tl: BoxTopLeft(tl),
            br: BoxBottomRight(br),
            collider: Collider::cuboid(w / 2., h / 2.),
            transform_bundle: TransformBundle::from_transform(Transform::from_xyz(
                tl.x + cx,
                tl.y + cy,
                10.,
            )),
            ..default()
        })
        .with_children(|cb| {
            // TOP
            cb.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(x_mesh.clone().into()).into(),
                transform: Transform::from_xyz(0.0, hy - 1., 0.0),
                material: materials.add(ColorMaterial {
                    texture: Some(texture_handles.chalk_line_horizontal.clone()),
                    ..default()
                }),
                ..default()
            });
            // LEFT
            cb.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(y_mesh.clone().into()).into(),
                transform: Transform::from_xyz(-hx + 1., 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_z(PI / 2.)),
                material: materials.add(ColorMaterial {
                    texture: Some(texture_handles.chalk_line_horizontal.clone()),
                    ..default()
                }),
                ..default()
            });
            // RIGHT
            cb.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(y_mesh.clone().into()).into(),
                transform: Transform::from_xyz(hx - 1., 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_z(-PI / 2.)),
                material: materials.add(ColorMaterial {
                    texture: Some(texture_handles.chalk_line_horizontal.clone()),
                    ..default()
                }),
                ..default()
            });
            // BOTTOM
            cb.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(x_mesh.clone().into()).into(),
                transform: Transform::from_xyz(0.0, -hy + 1., 0.0)
                    .with_rotation(Quat::from_rotation_z(PI)),
                material: materials.add(ColorMaterial {
                    texture: Some(texture_handles.chalk_line_horizontal.clone()),
                    ..default()
                }),
                ..default()
            });
        })
        .id()
}

fn setup_level(
    mut commands: Commands,
    texture_handles: Res<TextureHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const FLOOR_0: f32 = -5.;
    const FLOOR_0_BOTTOM: f32 = -10.;
    create_box(
        &mut commands,
        Vec2::new(0., FLOOR_0),
        Vec2::new(15., FLOOR_0_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

    create_box(
        &mut commands,
        Vec2::new(15., FLOOR_0 + 2.),
        Vec2::new(20., FLOOR_0_BOTTOM + 2.),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

    create_box(
        &mut commands,
        Vec2::new(20., FLOOR_0),
        Vec2::new(30., FLOOR_0_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

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
    let e1 = spawn_enemy(&mut commands, &texture_handles, Vec3::new(10.0, 0.0, 10.0));
    commands.entity(e1).insert(EnemyMover {dir: 1.});

    spawn_enemy(&mut commands, &texture_handles, Vec3::new(5.0, 0.0, 10.0));
}

fn despawn_level(commands: Commands, query: Query<Entity, With<InGameItem>>) {
    despawn_with(commands, query)
}

fn actor_fall_out(mut commands: Commands, query: Query<(&Transform, Entity), With<Actor>>) {
    for (transfrorm, entity) in &query {
        if transfrorm.translation.y < DEATHPLANE {
            commands.entity(entity).insert(ActorDead);
        }
    }
}
