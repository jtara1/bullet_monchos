use crate::components::*;

use bevy::prelude::*;

pub struct Label { pub label_type: LabelType }
#[derive(Debug)]
pub enum LabelType {
    FPS,
}

pub fn create_labels(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("fonts/A-Space Black Demo.otf");
    let font_size: f32 = 50.;

    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: font_handle.clone(),
                            font_size,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: "123.123".to_string(),
                        style: TextStyle {
                            font: font_handle.clone(),
                            font_size,
                            color: Color::GOLD,
                            ..Default::default()
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(LabelType::FPS);
}
