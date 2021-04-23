use bevy::prelude::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

use crate::{LabelType, Player, Health, Score};


pub fn update_labels(
    diagnostics: Res<Diagnostics>,
    mut ui_query: Query<(&mut Text, &LabelType)>,
    player_query: Query<(&Player, &Health)>,
    score: Res<Score>,
) {
    for (mut text, label_type) in ui_query.iter_mut() {
        match label_type {
            LabelType::Row1 => {
                if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                    if let Some(average) = fps.average() {
                        let index = (&text.sections).len() - 1;
                        text.sections[index].value = format!("{:.0}", average);
                    }
                }
            }
            LabelType::Row2 => {
                if let Ok((_player, health)) = player_query.single() {
                    text.sections[1].value = format!("{:.0}", health.current());
                }

                let index = (&text.sections).len() - 1;
                text.sections[index].value = score.0.to_string();
            }
        }
    }
}