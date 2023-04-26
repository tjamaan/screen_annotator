use bevy::{core_pipeline::clear_color::ClearColorConfig, input::common_conditions, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Screen Annotator".to_string(),
                //transparent: true,
                composite_alpha_mode: bevy::window::CompositeAlphaMode::PostMultiplied,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_systems(
            (
                start_drag.run_if(common_conditions::input_just_pressed(MouseButton::Left)),
                drag.run_if(resource_exists::<DragInProgress>()),
                end_drag.run_if(common_conditions::input_just_released(MouseButton::Left)),
            )
                .chain(),
        )
        .add_systems((destroy_annotation_sprites, create_annotation_sprites).chain())
        .run();
}

#[derive(Resource)]
struct DragInProgress {
    origin: Vec2,
    entity: Entity,
}

#[derive(Component, Debug)]
enum Annotation {
    Rect { start: Vec2, end: Vec2 },
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgba(0., 0., 0., 0.)),
        },
        ..default()
    });
}

fn start_drag(
    mut commands: Commands,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let win = q_window.single();
    let Some(cur_pos) = win.cursor_position() else {return;};
    let (camera, camera_transform) = q_camera.single();
    let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cur_pos) else {return;};
    println!("Start drag: {}", world_pos);
    let entitiy = commands
        .spawn(Annotation::Rect {
            start: world_pos,
            end: world_pos,
        })
        .id();
    commands.insert_resource(DragInProgress {
        origin: world_pos,
        entity: entitiy,
    });
}

fn drag(
    drag_in_progress: Res<DragInProgress>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut q_annotation: Query<&mut Annotation>,
) {
    let win = q_window.single();
    let Some(cur_pos) = win.cursor_position() else {return;};
    let (camera, camera_transform) = q_camera.single();
    let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cur_pos) else {return;};
    println!("Drag: {}", world_pos);
    let mut annotation = q_annotation.get_mut(drag_in_progress.entity).unwrap();
    match *annotation {
        Annotation::Rect { ref mut end, .. } => {
            *end = world_pos;
        }
    };
}

fn end_drag(mut commands: Commands) {
    println!("End drag");
    commands.remove_resource::<DragInProgress>();
}

#[derive(Component)]
struct AnnotationSprite;

fn destroy_annotation_sprites(
    mut commands: Commands,
    q_annotation_sprites: Query<Entity, With<AnnotationSprite>>,
) {
    let mut i = 0;
    for entity in &q_annotation_sprites {
        commands.entity(entity).despawn_recursive();
        i += 1;
    }
}

fn create_annotation_sprites(mut commands: Commands, q_annotations: Query<&Annotation>) {
    for annotation in &q_annotations {
        match annotation {
            Annotation::Rect { start, end } => {
                let top_left = Vec2::new(start.x.min(end.x), start.y.min(end.y));
                let bottom_right = Vec2::new(start.x.max(end.x), start.y.max(end.y));

                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::GREEN,
                            custom_size: Some(bottom_right - top_left),
                            anchor: bevy::sprite::Anchor::BottomLeft,
                            ..default()
                        },
                        transform: Transform::from_translation(top_left.extend(0.)),
                        ..default()
                    },
                    AnnotationSprite,
                ));
            }
        }
    }
}
