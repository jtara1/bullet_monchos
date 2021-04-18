mod greet;

use bevy::prelude::*;
use bevy::window;

use crate::greet::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(greet::GreetPlugin)
        .run();
}
