use godot::engine::{Control, ControlVirtual, Label};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct Hud {
    score_label: Option<Gd<Label>>,
    lives_label: Option<Gd<Label>>,

    #[base]
    control: Base<Control>,
}

#[godot_api]
impl ControlVirtual for Hud {
    fn init(control: Base<Control>) -> Self {
        Self {
            score_label: None,
            lives_label: None,
            control,
        }
    }

    fn ready(&mut self) {
        self.score_label = Some(self.control.get_node_as::<Label>("Score"));
        self.lives_label = Some(self.control.get_node_as::<Label>("Lives"));
    }
}

#[godot_api]
impl Hud {
    #[func]
    pub fn set_score_label(&mut self, score: u32) {
        if let Some(score_label) = &mut self.score_label {
            score_label.set_text(format!("SCORE: {}", score).into());
        }
    }

    #[func]
    pub fn set_lives_label(&mut self, lives: i8) {
        if let Some(lives_label) = &mut self.lives_label {
            lives_label.set_text(format!("{lives}").into());
        }
    }
}
