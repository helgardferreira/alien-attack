use crate::player::Player;
use godot::engine::{Area2D, Area2DVirtual};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Enemy {
    #[export]
    pub speed: f32,

    #[base]
    pub area: Base<Area2D>,
}

#[godot_api]
impl Area2DVirtual for Enemy {
    fn init(area: Base<Area2D>) -> Self {
        Self { area, speed: 700.0 }
    }

    fn ready(&mut self) {
        let enemy = self.area.clone().cast::<Enemy>();

        self.area.connect(
            "body_entered".into(),
            Callable::from_object_method(enemy, "on_body_entered"),
        );
    }

    fn physics_process(&mut self, delta: f64) {
        let pos = self.area.get_global_position();
        self.area
            .set_global_position(pos - Vector2::new(self.speed * delta as f32, 0.0));
    }
}

#[godot_api]
impl Enemy {
    pub fn die(&mut self) {
        self.area.queue_free();
        self.area.emit_signal("died".into(), &[]);
    }

    pub fn free(&mut self) {
        self.area.queue_free();
    }

    #[func]
    fn on_body_entered(&mut self, body: Gd<Node2D>) {
        if body.try_cast::<Player>().is_some() {
            self.die();
            self.area.emit_signal("attack".into(), &[]);
        };
    }

    #[signal]
    fn attack() {}

    #[signal]
    fn died() {}
}
