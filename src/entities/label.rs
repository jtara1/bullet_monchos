use crate::components::*;

use bevy::prelude::*;

pub struct Label { pub label_type: LabelType }
pub enum LabelType {
    Fps,
}

pub fn create_labels(mut commands: Commands) {
    // let font_handle = asset_server.get_handle("fonts/abc.ttf");

    commands
        .spawn_bundle(TextBundle {
            node: Node { size: Vec2::new(100., 100.) },
            style: Style {
                // align_self: AlignSelf::FlexEnd,
                // position_type: PositionType::Absolute,
                // position: Rect {
                //     top: Val::Px(font_size * 2.0),
                //     ..Default::default()
                // },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(LabelType::Fps);
}
