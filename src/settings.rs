use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Resource)]
pub struct Settings {
    pub movement_speed: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            movement_speed: 3.14,
        }
    }
}

