use crate::enemy::Enemy;
use godot::engine::{Area2D, Area2DVirtual, VisibleOnScreenNotifier2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Rocket {
    #[export]
    pub speed: f32,

    #[base]
    pub area: Base<Area2D>,
}

#[godot_api]
impl Area2DVirtual for Rocket {
    fn init(area: Base<Area2D>) -> Self {
        Self {
            area,
            speed: 1000.0,
        }
    }

    fn ready(&mut self) {
        // Might not be necessary to cache the visible on screen notifier
        let mut visible_notifier = self
            .area
            .get_node_as::<VisibleOnScreenNotifier2D>("VisibleNotifier");
        let rocket = self.area.clone().cast::<Rocket>();
        visible_notifier.connect(
            "screen_exited".into(),
            Callable::from_object_method(rocket, "on_screen_exited"),
        );

        let rocket = self.area.clone().cast::<Rocket>();

        self.area.connect(
            "area_entered".into(),
            Callable::from_object_method(rocket, "on_area_entered"),
        );
    }

    fn physics_process(&mut self, delta: f64) {
        let pos = self.area.get_global_position();
        self.area
            .set_global_position(pos + Vector2::new(self.speed * delta as f32, 0.0));
    }
}

#[godot_api]
impl Rocket {
    #[func]
    fn on_screen_exited(&mut self) {
        self.area.queue_free();
    }

    #[func]
    fn on_area_entered(&mut self, node: Gd<Node2D>) {
        if let Some(mut enemy) = node.clone().try_cast::<Enemy>() {
            self.area.queue_free();
            enemy.bind_mut().die();
        }
    }
}
