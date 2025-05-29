use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Resource)]
pub struct KeyBindings {
    pub forward: KeyCode,
    pub backward: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub jump: KeyCode,
    pub sprint: KeyCode,
    pub open_inventory: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        KeyBindings {
            forward: KeyCode::KeyW,
            backward: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            jump: KeyCode::Space,
            sprint: KeyCode::ShiftLeft,
            open_inventory: KeyCode::Tab,
        }
    }
}

