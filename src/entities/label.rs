use crate::components::*;

use bevy::prelude::*;

pub struct Label { pub label_type: LabelType }
#[derive(Debug)]
pub enum LabelType {
    Row1,
    Row2,
}

pub fn create_labels(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("fonts/OpenSans-Bold.ttf");
    let font_size: f32 = 40.;

    commands
        // Clone, Shoot, FPS
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Clone: ".to_string(),
                        style: TextStyle {
                            font: font_handle.clone(),
                            font_size,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "F".to_string(),
                        style: TextStyle {
                            font: font_handle.clone(),
                            font_size,
                            color: Color::GOLD,
                        },
                    },
                    TextSection {
                        value: "    Shoot: ".to_string(),
                        style: TextStyle {
                            font: font_handle.clone(),
                            font_size,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "Space".to_string(),
                        style: TextStyle {
                            font: font_handle.clone(),
                            font_size,
                            color: Color::GOLD,
                        },
                    },
                    TextSection {
                        value: "    FPS: ".to_string(),
                        style: TextStyle {
                            font: font_handle.clone(),
                            font_size,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font_handle.clone(),
                            font_size,
                            color: Color::GOLD,
                            ..Default::default()
                        },
                    },
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Bottom,
                    horizontal: HorizontalAlign::Center,
                },
                ..Default::default()
            },
            transform: Transform::from_xyz(0., -200., 4.),
            ..Default::default()
        })
        .insert(LabelType::Row1);

    commands
        // Health
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Health: ".to_string(),
                        style: TextStyle {
                            font: font_handle.clone(),
                            font_size,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font_handle.clone(),
                            font_size,
                            color: Color::RED,
                        },
                    },
                    TextSection {
                        value: "    Score: ".to_string(),
                        style: TextStyle {
                            font: font_handle.clone(),
                            font_size,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font_handle.clone(),
                            font_size,
                            color: Color::GOLD,
                        },
                    },
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Bottom,
                    horizontal: HorizontalAlign::Center,
                },
                ..Default::default()
            },
            transform: Transform::from_xyz(0., -275., 4.),
            ..Default::default()
        })
        .insert(LabelType::Row2);
}
