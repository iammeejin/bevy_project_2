use bevy::{
    prelude::*,
    text::{BreakLineOn, Text2dBounds},
};
use bevy::window::PrimaryWindow;
use rand::prelude::*;


pub const NUMBER_OF_TEXT_BOXES: usize = 4;
pub const TEXT_BOXES_SPEED: f32 = 200.0;
pub const TEXT_BOXES_SIZE: f32 = 64.0;
pub const CAMERA_SPEED_PER_SEC: f32 = 1.0;

pub fn main() {
    App::new()
    // Bevy Plugins
    .add_plugins(DefaultPlugins)
    //Startup System
    .add_systems(Startup,spawn_camera)
    .add_systems(Startup, setup_ocean)
    .add_systems(Startup, spawn_text_boxes)
    .add_systems(Update, camera_control_pan)
    .add_systems(Update, camera_control_zoom)
    .add_systems(Update, text_boxes_movement)
    .add_systems(Update, update_text_boxes_direction)
    .add_systems(Update, confine_text_boxes_movement)
    .run();
}

#[derive(Component)]
pub struct TextBox {
    pub direction: Vec2,
    pub is_hovered: bool,
}

#[derive(Component)]
pub struct Music;


pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let camera_height = 1000.0;
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(window.width() / 2.0, window.height() / 2.0, camera_height))
            .looking_at(Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0), Vec3::Y),
        ..Default::default()
    });
}

pub fn setup_ocean(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(window.width(), window.height())),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0)), // Place at the center
        ..Default::default()
    });
}

pub fn camera_control_pan(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera2d>>,
) {
    for mut transform in query.iter_mut() {
        let pan_speed = 10.0;

        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += pan_speed;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= pan_speed;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= pan_speed;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += pan_speed;
        }
    }
}

pub fn camera_control_zoom(kb: Res<Input<KeyCode>>, time: Res<Time>, mut query: Query<&mut OrthographicProjection, With<Camera2d>>) {
    let dist = CAMERA_SPEED_PER_SEC * time.delta().as_secs_f32();

    for mut projection in query.iter_mut() {
        let mut log_scale = projection.scale.ln();

        if kb.pressed(KeyCode::PageUp) {
            log_scale -= dist;
        }
        if kb.pressed(KeyCode::PageDown) {
            log_scale += dist;
        }

        projection.scale = log_scale.exp();
    }
}


pub fn spawn_text_boxes(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let camera_height = 1000.0;
    let text_box_height = camera_height - 10.0;
    let text_values = ["Josh", "Almie", "redbeard", "github.com"];


    for i in 0..NUMBER_OF_TEXT_BOXES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        let text_value = text_values[i % text_values.len()];

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.98, 0.92, 0.84),
                    custom_size: Some(Vec2::new(200.0, 80.0)),
                    ..default()
                },
                    transform: Transform::from_xyz(random_x,random_y,text_box_height),
                    ..default()
            },
            TextBox {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                is_hovered: false,
            },
        ))

        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: text_value.into(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 32.0,
                            color: Color::BLACK,
                        },
                    }],
                    linebreak_behavior:BreakLineOn::WordBoundary,
                    alignment: TextAlignment::Left,
                },
                text_2d_bounds: Text2dBounds {
                    // Wrap text in the rectangle
                    size: Vec2::new(200.0, 80.0),
                },
                // ensure the text is drawn on top of the box
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            });
        });
    }
}


pub fn text_boxes_movement(
    mut text_boxes_query: Query<(&mut Transform, &TextBox)>,
    time: Res<Time>,
) {
    for (mut transform, textbox) in text_boxes_query.iter_mut() {
        if !textbox.is_hovered {
            let direction = Vec3::new(textbox.direction.x, textbox.direction.y, 0.0);
            transform.translation += direction * TEXT_BOXES_SPEED * time.delta_seconds();
        }
    }
}

pub fn update_text_boxes_direction(
    mut text_boxes_query: Query<(&Transform, &mut TextBox)>,
    window_query: Query<&Window, With <PrimaryWindow>>,
){
    let window = window_query.get_single().unwrap();

    let half_text_box_size = TEXT_BOXES_SIZE / 2.0; //32.0
        let x_min = 0.0 + half_text_box_size;
        let x_max = window.width() - half_text_box_size;
        let y_min = 0.0 + half_text_box_size;
        let y_max = window.height() - half_text_box_size;

    for (transform, mut textbox) in text_boxes_query.iter_mut() {
        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            textbox.direction.x *= -1.0;
        }
        if translation.y < y_min || translation.y > y_max {
            textbox.direction.y *= -1.0;
        }
        }
    }


pub fn confine_text_boxes_movement(
    mut text_boxes_query: Query<&mut Transform, With<TextBox>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    let half_text_box_size: f32 = TEXT_BOXES_SIZE / 2.0;
    let x_min: f32 = 0.0 + half_text_box_size;
    let x_max: f32 = window.width() - half_text_box_size;
    let y_min: f32 = 0.0 + half_text_box_size;
    let y_max: f32 = window.height() - half_text_box_size;

    for mut transform in text_boxes_query.iter_mut() {
        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;

    }
}

