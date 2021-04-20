use crate::components::*;

use bevy::prelude::*;

pub struct Label { pub label_type: LabelType }
pub enum LabelType {
    Fps,
}

pub fn create_labels(mut commands: Commands) {
    // let font_handle = asset_server.get_handle("fonts/abc.ttf");
    let font_size: f32 = 50.;

    commands
        .spawn_bundle(TextBundle {
            node: Node { size: Vec2::new(1000., 1000.) },
            style: Style {
                // align_self: AlignSelf::FlexEnd,
                // position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(font_size * 2.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: String::from("FPS placeholder"),
                    ..Default::default()
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
            ..Default::default()
        })
        .insert(LabelType::Fps);

    println!("create_labels(): success");
}
