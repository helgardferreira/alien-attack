use godot::prelude::*;

pub struct InputDirections {
    pub right: bool,
    pub left: bool,
    pub up: bool,
    pub down: bool,
}

impl InputDirections {
    pub fn new(input: Gd<Input>) -> Self {
        InputDirections {
            right: input.is_action_pressed("move_right".into()),
            left: input.is_action_pressed("move_left".into()),
            up: input.is_action_pressed("move_up".into()),
            down: input.is_action_pressed("move_down".into()),
        }
    }
}
