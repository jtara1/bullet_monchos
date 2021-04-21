use bevy::prelude::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

use crate::{LabelType};


pub fn update_labels(
    diagnostics: Res<Diagnostics>,
    mut query: Query<(&mut Text, &LabelType)>,
) {
    if let Ok((mut text, label_type)) = query.single_mut() {
        match label_type {
            LabelType::FPS => {
                if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                    if let Some(average) = fps.average() {
                        text.sections[1].value = format!("{:.2}", average);
                        println!("FPS: {}", average);
                    }
                }
            }
        }
    }
}