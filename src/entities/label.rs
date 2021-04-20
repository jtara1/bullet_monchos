use crate::components::*;

use bevy::prelude::*;

pub struct Label { pub label_type: LabelType }
pub enum LabelType {
    Fps,
}

pub fn create_labels(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.get_handle("fonts/A-Space Black Demo.ttf");
    let font_size: f32 = 50.;

    commands
        .spawn_bundle(TextBundle {
            node: Node { size: Vec2::new(font_size, font_size) },
            style: Style {
                // align_self: AlignSelf::FlexEnd,
                // position_type: PositionType::Absolute,
                position: Rect {
                    // top: Val::Px(font_size * 2.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: String::from("FPS placeholder"),
                    style: TextStyle {
                        font: font_handle.clone(),
                        font_size,
                        color: Color::from([255., 255., 255., 1.])
                    }
                }],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center
                },
                ..Default::default()
            },
            calculated_size: CalculatedSize {
                size: Size {
                    width: 100.,
                    height: 100.
                }
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 3.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(LabelType::Fps);

    println!("create_labels(): success");
}
