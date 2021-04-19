use bevy::prelude::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

use crate::{LabelType, Label};


pub fn update_labels(
    diagnostics: Res<Diagnostics>,
    mut query: Query<(&mut TextBundle, &Label)>,
) {
    for (mut text_bundle, label) in query.iter_mut() {
        println!("label - queried");
        match label.label_type {
            LabelType::Fps => {
                println!("label - is fps");
                if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                    if let Some(average) = fps.average() {
                        let text_section = TextSection {
                            value: format!("FPS: {:.0}", average),
                            ..Default::default()
                        };

                        match text_bundle.text.sections.get(0) {
                            None => panic!("empty text section for fps ui"),
                            _ => ()
                        }

                        println!("label sys {}", average);
                        text_bundle.text.sections[0] = text_section;
                    }
                }
            }
        }
    }
}