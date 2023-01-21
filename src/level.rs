use std::f32::consts::PI;

use bevy::{prelude::*, render::mesh::VertexAttributeValues, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionHelpers, IntoConditionalSystem};

use crate::{
    enemy::spawn_enemy,
    player::spawn_player,
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
            // Edge colliders
            const EC_WIDTH: f32 = 0.1;
            const EC_HEIGHT: f32 = 1.0;
            cb.spawn((
                Collider::cuboid(EC_WIDTH / 2., EC_HEIGHT / 2.),
                CollisionGroups::new(Group::GROUP_1, Group::GROUP_1),
            ))
            .insert(TransformBundle::from_transform(Transform::from_xyz(
                -(w + EC_WIDTH) / 2.,
                (h + EC_HEIGHT) / 2.,
                10.0,
            )));

            cb.spawn((
                Collider::cuboid(EC_WIDTH / 2., EC_HEIGHT / 2.),
                CollisionGroups::new(Group::GROUP_1, Group::GROUP_1),
            ))
            .insert(TransformBundle::from_transform(Transform::from_xyz(
                (w + EC_WIDTH) / 2.,
                (h + EC_HEIGHT) / 2.,
                10.0,
            )));

            // Graphics
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
    const FLOOR_0: f32 = -10.;
    const FLOOR_1: f32 = 0.;
    const FLOOR_0_BOTTOM: f32 = -15.;
    const FLOOR_1_BOTTOM: f32 = -5.;

    // const MAX_JUMP: f32 = 12.;

    spawn_player(
        &mut commands,
        &texture_handles,
        Vec3::new(80., FLOOR_1 + 1., 10.),
    );

    // first static enemy
    spawn_enemy(
        &mut commands,
        &texture_handles,
        Vec3::new(10.0, FLOOR_0 + 1.0, 10.0),
        false,
    );

    create_box(
        &mut commands,
        Vec2::new(0., FLOOR_0),
        Vec2::new(15., FLOOR_0_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

    // first moving enemy
    spawn_enemy(
        &mut commands,
        &texture_handles,
        Vec3::new(25.0, FLOOR_0 + 1.0, 10.0),
        true,
    );

    create_box(
        &mut commands,
        Vec2::new(20., FLOOR_0),
        Vec2::new(30., FLOOR_0_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

    // higher level

    create_box(
        &mut commands,
        Vec2::new(30., FLOOR_1),
        Vec2::new(50., FLOOR_0_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

    // big jump down from higher up

    create_box(
        &mut commands,
        Vec2::new(62., FLOOR_0),
        Vec2::new(75., FLOOR_0_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

    // jump to higher level with gap
    spawn_enemy(
        &mut commands,
        &texture_handles,
        Vec3::new(82.5, FLOOR_1 + 1., 10.),
        false,
    );
    create_box(
        &mut commands,
        Vec2::new(80., FLOOR_1),
        Vec2::new(85., FLOOR_0_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

    // jump down past gap onto small platform with moving enemy

    spawn_enemy(
        &mut commands,
        &texture_handles,
        Vec3::new(96., FLOOR_0 + 1., 10.),
        true,
    );
    create_box(
        &mut commands,
        Vec2::new(95., FLOOR_0),
        Vec2::new(98., FLOOR_0_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

    // little gap little platform

    create_box(
        &mut commands,
        Vec2::new(102., FLOOR_0),
        Vec2::new(105., FLOOR_0_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

    // overhang
    spawn_enemy(
        &mut commands,
        &texture_handles,
        Vec3::new(115., FLOOR_0 + 1., 10.),
        true,
    );

    create_box(
        &mut commands,
        Vec2::new(110., FLOOR_0),
        Vec2::new(120., FLOOR_0_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

    create_box(
        &mut commands,
        Vec2::new(115., FLOOR_1),
        Vec2::new(120., FLOOR_1_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

    create_box(
        &mut commands,
        Vec2::new(120., FLOOR_1),
        Vec2::new(130., FLOOR_0_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

    // longish platform
    spawn_enemy(
        &mut commands,
        &texture_handles,
        Vec3::new(141., FLOOR_0 + 1., 10.),
        true,
    );

    spawn_enemy(
        &mut commands,
        &texture_handles,
        Vec3::new(144., FLOOR_0 + 1., 10.),
        true,
    );

    spawn_enemy(
        &mut commands,
        &texture_handles,
        Vec3::new(147., FLOOR_0 + 1., 10.),
        true,
    );

    spawn_enemy(
        &mut commands,
        &texture_handles,
        Vec3::new(159., FLOOR_0 + 1., 10.),
        false,
    );

    create_box(
        &mut commands,
        Vec2::new(140., FLOOR_0),
        Vec2::new(160., FLOOR_0_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

    // steps
    spawn_enemy(
        &mut commands,
        &texture_handles,
        Vec3::new(162., FLOOR_0 + 4., 10.),
        false,
    );
    create_box(
        &mut commands,
        Vec2::new(160., FLOOR_0 + 3.),
        Vec2::new(164., FLOOR_0_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

    spawn_enemy(
        &mut commands,
        &texture_handles,
        Vec3::new(166., FLOOR_0 + 7., 10.),
        false,
    );
    create_box(
        &mut commands,
        Vec2::new(164., FLOOR_0 + 6.),
        Vec2::new(168., FLOOR_0_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

    spawn_enemy(
        &mut commands,
        &texture_handles,
        Vec3::new(170., FLOOR_0 + 10., 10.),
        false,
    );
    create_box(
        &mut commands,
        Vec2::new(168., FLOOR_0 + 9.),
        Vec2::new(172., FLOOR_0_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

    spawn_enemy(
        &mut commands,
        &texture_handles,
        Vec3::new(174., FLOOR_0 + 13., 10.),
        false,
    );
    create_box(
        &mut commands,
        Vec2::new(172., FLOOR_0 + 12.),
        Vec2::new(176., FLOOR_0_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

    // big gap, finish platform
    create_box(
        &mut commands,
        Vec2::new(188., FLOOR_0),
        Vec2::new(220., FLOOR_0_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );
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
