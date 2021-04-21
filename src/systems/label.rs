use bevy::prelude::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

use crate::{LabelType};


pub fn update_labels(
    diagnostics: Res<Diagnostics>,
    mut query: Query<(&mut Text, &LabelType)>,
) {
    for (mut text, label_type) in query.iter_mut() {
        // text.sections[1].value = "42.42".to_string();

        match label_type {
            LabelType::FPS => {
                // println!("found label type");
                if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                    // println!("found fps");

                    if let Some(average) = fps.average() {
                        // println!("found fps average");

                        text.sections[1].value = format!("{:.0}", average);
                    }
                }
            }
        }
    }
}