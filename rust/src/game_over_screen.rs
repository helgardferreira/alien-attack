use godot::engine::{Button, Control, ControlVirtual, Label};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct GameOverScreen {
    score_label: Option<Gd<Label>>,

    #[base]
    control: Base<Control>,
}

#[godot_api]
impl ControlVirtual for GameOverScreen {
    fn init(control: Base<Control>) -> Self {
        Self {
            control,
            score_label: None,
        }
    }

    fn ready(&mut self) {
        self.score_label = Some(self.control.get_node_as("Panel/Score"));

        let mut button: Gd<Button> = self.control.get_node_as("Panel/RetryButton");

        let screen = self.control.clone().cast::<GameOverScreen>();
        button.connect(
            "pressed".into(),
            Callable::from_object_method(screen, "on_retry_button_pressed"),
        );
    }
}

#[godot_api]
impl GameOverScreen {
    #[func]
    fn on_retry_button_pressed(&mut self) {
        if let Some(mut tree) = self.control.get_tree() {
            tree.reload_current_scene();
        }
    }

    pub fn set_score(&mut self, score: u32) {
        if let Some(score_label) = &mut self.score_label {
            score_label.set_text(format!("SCORE: {score}").into());
        }
    }
}
