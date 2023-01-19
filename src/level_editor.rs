use bevy::prelude::*;
use bevy_rapier2d::{na::Vector2, prelude::Collider};
use iyes_loopless::{
    prelude::{AppLooplessStateExt, IntoConditionalSystem},
    state::NextState,
};

use crate::{level::create_box, states::GameState, util::despawn_with, TextureHandles};
pub struct LevelEditorPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EditorTool {
    Expand,
    Move,
    Shrink,
    Select,
}

#[derive(Component)]
struct LevelEditorItem;

#[derive(Component)]
struct EditorSelected;

#[derive(Component)]
struct Crosshair;

impl Plugin for LevelEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(EditorTool::Select)
            .add_enter_system(GameState::LevelEditor, setup_level_editor)
            .add_exit_system(GameState::LevelEditor, cleanup_level_editor)
            .add_system(editor_tool_expand.run_in_state(GameState::LevelEditor))
            .add_system(level_editor_input.run_in_state(GameState::LevelEditor));
    }
}

fn setup_level_editor(mut commands: Commands, texture_handles: Res<TextureHandles>) {
    commands
        .spawn(SpriteBundle {
            texture: texture_handles.crosshair.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            ..default()
        })
        .insert(Crosshair)
        .insert(LevelEditorItem);
}

fn deselect(commands: &mut Commands, q_currently_selected: &Query<Entity, With<EditorSelected>>) {
    for entity in q_currently_selected {
        commands.entity(entity).remove::<EditorSelected>();
    }
}

fn level_editor_input(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut q_camera: Query<(&Camera2d, &mut Transform), Without<Crosshair>>,
    mut q_crosshair: Query<(&Crosshair, &mut Transform), Without<Camera2d>>,

    q_currently_selected: Query<Entity, With<EditorSelected>>,

    // create box schtuff
    texture_handles: Res<TextureHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut t_cam = q_camera.single_mut().1;
    let mut t_cross = q_crosshair.single_mut().1;

    if input.just_pressed(KeyCode::D) {
        t_cam.translation.x += 1.;
        t_cross.translation.x += 1.;
    }

    if input.just_pressed(KeyCode::A) {
        t_cam.translation.x += -1.;
        t_cross.translation.x += -1.;
    }

    if input.just_pressed(KeyCode::W) {
        t_cross.translation.y += 1.;
    }

    if input.just_pressed(KeyCode::S) {
        t_cross.translation.y += -1.;
    }

    if input.just_pressed(KeyCode::B) {
        let new_box = create_box(
            &mut commands,
            Vec2::new(t_cross.translation.x, t_cross.translation.y),
            Vec2::new(t_cross.translation.x + 1.0, t_cross.translation.y - 1.0),
            &texture_handles,
            &mut meshes,
            &mut materials,
        );

        deselect(&mut commands, &q_currently_selected);

        commands.entity(new_box).insert(EditorSelected);
    }

    if input.any_just_pressed([KeyCode::Escape, KeyCode::Key0]) {
        deselect(&mut commands, &q_currently_selected);
        commands.insert_resource(NextState(EditorTool::Select));
    }

    if input.just_pressed(KeyCode::Key1) {
        // move
        commands.insert_resource(NextState(EditorTool::Move));
    }

    if input.just_pressed(KeyCode::Key2) {
        // expand
        commands.insert_resource(NextState(EditorTool::Expand));
    }

    if input.just_pressed(KeyCode::Key3) {
        // shrink
        commands.insert_resource(NextState(EditorTool::Shrink));
    }
}

fn editor_tool_expand(
    mut q: Query<&mut Collider, With<EditorSelected>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::E) {
        q.get_single_mut()
            .unwrap()
            .as_cuboid_mut()
            .unwrap()
            .raw
            .half_extents = Vector2::new(2.0, 2.0);
    }
}

fn cleanup_level_editor(commands: Commands, q: Query<Entity, With<LevelEditorItem>>) {
    despawn_with(commands, q);
}
