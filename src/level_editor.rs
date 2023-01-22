use bevy::prelude::*;
use bevy_rapier2d::{na::Vector2, prelude::Collider};
use iyes_loopless::{
    prelude::{AppLooplessStateExt, ConditionHelpers, IntoConditionalSystem},
    state::NextState,
};

use crate::{level::create_box, states::GameState, util::despawn_with, TextureHandles};
pub struct LevelEditorPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EditorTool {
    Expand,
    Move,
    Shrink,
    Select, // And create
}

#[derive(Component)]
struct LevelEditorItem;

#[derive(Component)]
struct EditorSelected;

#[derive(Component)]
struct Crosshair;

#[derive(Component)]
struct ToolText;

impl Plugin for LevelEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(EditorTool::Select)
            .add_enter_system(GameState::LevelEditor, setup_level_editor)
            .add_exit_system(GameState::LevelEditor, cleanup_level_editor)
            // input
            .add_system(
                editor_tool_expand
                    .run_in_state(GameState::LevelEditor)
                    .run_in_state(EditorTool::Expand),
            )
            .add_system(
                move_crosshair
                    .run_in_state(GameState::LevelEditor)
                    .run_in_state(EditorTool::Select),
            )
            .add_system(
                editor_create_box
                    .run_in_state(GameState::LevelEditor)
                    .run_in_state(EditorTool::Select),
            )
            .add_system(level_editor_input.run_in_state(GameState::LevelEditor))
            // ui
            .add_enter_system(
                EditorTool::Select,
                ui_select_enter.run_in_state(GameState::LevelEditor),
            )
            .add_enter_system(EditorTool::Move, ui_move_enter)
            .add_enter_system(EditorTool::Expand, ui_expand_enter)
            .add_enter_system(EditorTool::Shrink, ui_shrink_enter);
    }
}

fn ui_select_enter(mut q_tool_text: Query<&mut Text, With<ToolText>>) {
    q_tool_text.single_mut().sections[0].value = "SELECT".into();
}

fn ui_move_enter(mut q_tool_text: Query<&mut Text, With<ToolText>>) {
    q_tool_text.single_mut().sections[0].value = "MOVE".into();
}

fn ui_expand_enter(mut q_tool_text: Query<&mut Text, With<ToolText>>) {
    q_tool_text.single_mut().sections[0].value = "EXPAND".into();
}

fn ui_shrink_enter(mut q_tool_text: Query<&mut Text, With<ToolText>>) {
    q_tool_text.single_mut().sections[0].value = "SHRINK".into();
}

fn setup_level_editor(
    mut commands: Commands,
    texture_handles: Res<TextureHandles>,
    mut q_camera: Query<&mut Transform, (With<Camera2d>, Without<Crosshair>)>,
    asset_server: Res<AssetServer>,
) {
    let mut t_cam = q_camera.single_mut();

    t_cam.translation = Vec3::new(0., 0., 0.);

    commands
        .spawn(SpriteBundle {
            texture: texture_handles.crosshair.clone().unwrap(),
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            ..default()
        })
        .insert(Crosshair)
        .insert(LevelEditorItem);

    // ui
    commands
        .spawn(TextBundle {
            text: Text::from_section(
                "SELECT",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ),
            transform: Transform::from_xyz(0., 0., 10.0),
            ..default()
        })
        .insert(ToolText)
        .insert(LevelEditorItem);
}

fn deselect(commands: &mut Commands, q_currently_selected: &Query<Entity, With<EditorSelected>>) {
    for entity in q_currently_selected {
        commands.entity(entity).remove::<EditorSelected>();
    }
}

fn move_crosshair(
    mut q_camera: Query<&mut Transform, (With<Camera2d>, Without<Crosshair>)>,
    mut q_crosshair: Query<&mut Transform, (With<Crosshair>, Without<Camera2d>)>,
    input: Res<Input<KeyCode>>,
) {
    let mut t_cam = q_camera.single_mut();
    let mut t_cross = q_crosshair.single_mut();

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
}

fn editor_create_box(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    q_crosshair: Query<&mut Transform, (With<Crosshair>, Without<Camera2d>)>,

    q_currently_selected: Query<Entity, With<EditorSelected>>,

    texture_handles: Res<TextureHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let t_cross = q_crosshair.single();

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
}

fn level_editor_input(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,

    q_currently_selected: Query<Entity, With<EditorSelected>>,
) {
    if let Ok(_e) = q_currently_selected.get_single() {
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
    } else {
        if input.any_just_pressed([KeyCode::Key1, KeyCode::Key2, KeyCode::Key3]) {
            println!("NOTHING SELECTED!!");
        }
    }

    if input.just_pressed(KeyCode::Key0) {
        deselect(&mut commands, &q_currently_selected);
        commands.insert_resource(NextState(EditorTool::Select));
    }
}

fn editor_tool_expand(
    mut q: Query<(&mut Collider, &mut Transform), With<EditorSelected>>,
    input: Res<Input<KeyCode>>,
) {
    let (mut collider, mut transform) = q.get_single_mut().unwrap();
    let he = collider.as_cuboid_mut().unwrap().raw.half_extents;

    if input.just_pressed(KeyCode::D) {
        collider.as_cuboid_mut().unwrap().raw.half_extents = Vector2::new(he.x + 0.5, he.y);
        transform.translation.x += 0.5;
    }

    if input.just_pressed(KeyCode::A) {
        collider.as_cuboid_mut().unwrap().raw.half_extents = Vector2::new(he.x + 0.5, he.y);
        transform.translation.x -= 0.5;
    }

    if input.just_pressed(KeyCode::W) {
        collider.as_cuboid_mut().unwrap().raw.half_extents = Vector2::new(he.x, he.y + 0.5);
        transform.translation.y += 0.5;
    }

    if input.just_pressed(KeyCode::S) {
        collider.as_cuboid_mut().unwrap().raw.half_extents = Vector2::new(he.x, he.y + 0.5);
        transform.translation.y -= 0.5;
    }
}

fn cleanup_level_editor(commands: Commands, q: Query<Entity, With<LevelEditorItem>>) {
    despawn_with(commands, q);
}
