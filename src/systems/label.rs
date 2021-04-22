use bevy::prelude::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

use crate::{LabelType, Player, Health};


pub fn update_labels(
    diagnostics: Res<Diagnostics>,
    mut ui_query: Query<(&mut Text, &LabelType)>,
    player_query: Query<(&Player, &Health)>,
) {
    for (mut text, label_type) in ui_query.iter_mut() {
        match label_type {
            LabelType::FPS => {
                if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                    if let Some(average) = fps.average() {
                        let index = (&text.sections).len() - 1;
                        text.sections[index].value = format!("{:.0}", average);
                    }
                }
            }
            LabelType::Health => {
                if let Ok((_player, health)) = player_query.single() {
                    let index = (&text.sections).len() - 1;
                    text.sections[index].value = format!("{:.0}", health.current());
                }
            }
        }
    }
}