use crate::rocket::Rocket;

use self::utilities::InputDirections;
use godot::engine::{
    character_body_2d::MotionMode, AudioStreamPlayer2D, CharacterBody2D, CharacterBody2DVirtual,
    Time,
};
use godot::prelude::*;

mod utilities;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    #[export]
    speed: f32,
    #[export]
    time_between_shots: f32,
    next_shot_time: u64,

    sprite_offset: Vector2,
    rocket_scene: Gd<PackedScene>,
    rocket_container: Option<Gd<Node>>,
    player_shoot_sound: Option<Gd<AudioStreamPlayer2D>>,

    #[base]
    pub body: Base<CharacterBody2D>,
}

#[godot_api]
impl CharacterBody2DVirtual for Player {
    fn init(body: Base<CharacterBody2D>) -> Self {
        let mut new_body = body;
        new_body.set_motion_mode(MotionMode::MOTION_MODE_FLOATING);

        let sprite_offset = Vector2::new(36.0, 54.0);

        Self {
            body: new_body,

            speed: 600.0,
            time_between_shots: 0.15,
            next_shot_time: 0,

            sprite_offset,
            rocket_scene: PackedScene::new(),
            rocket_container: None,
            player_shoot_sound: None,
        }
    }

    fn ready(&mut self) {
        // Note: this is downcast during load() -- completely type-safe thanks to type inference!
        // If the resource does not exist or has an incompatible type, this panics.
        // There is also try_load() if you want to check whether loading succeeded.
        self.rocket_scene = load("res://scenes/rocket.tscn");
        self.rocket_container = Some(self.body.get_node_as("RocketContainer"));
        self.player_shoot_sound = Some(self.body.get_node_as("PlayerShootSound"));
    }

    fn process(&mut self, _delta: f64) {
        let shoot_pressed = Input::singleton().is_action_pressed("shoot".into());
        let elapsed_time = Time::singleton().get_ticks_msec();

        if shoot_pressed && self.next_shot_time <= elapsed_time {
            self.next_shot_time = elapsed_time + (self.time_between_shots * 1000.0).floor() as u64;

            self.shoot();
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        let directions = InputDirections::new(Input::singleton());

        self.move_player(directions);

        self.prevent_out_of_bounds();
    }
}

#[godot_api]
impl Player {
    /// Instances a rocket from the player
    fn shoot(&mut self) {
        let mut rocket = self.rocket_scene.instantiate_as::<Rocket>();
        let player_pos = self.body.get_global_position();
        rocket.set_global_position(player_pos + Vector2::new(70.0, 0.0));

        if let Some(rocket_container) = &mut self.rocket_container {
            rocket_container.add_child(rocket.upcast());
        }

        if let Some(player_shoot_sound) = &mut self.player_shoot_sound {
            player_shoot_sound.play();
        }
    }

    /// Moves the player based on input directions
    fn move_player(&mut self, d: InputDirections) {
        let v = self.body.get_velocity();

        if d.right || d.left || d.up || d.down {
            let mut x: f32 = 0.0;
            let mut y: f32 = 0.0;
            if d.right && d.left {
                x = 0.0;
            } else if d.right {
                x = self.speed;
            } else if d.left {
                x = -self.speed;
            }

            if d.up && d.down {
                y = 0.0;
            } else if d.up {
                y = -self.speed;
            } else if d.down {
                y = self.speed;
            }

            self.body.set_velocity(Vector2::new(x, y));
        } else if v.length() > 0.0 {
            self.body.set_velocity(Vector2::new(0.0, 0.0));
        };

        self.body.move_and_slide();
    }

    /// Prevents the [`Player`] from going out of bounds.
    fn prevent_out_of_bounds(&mut self) {
        let screen_size = self.body.get_viewport_rect().size;

        let pos = self.body.get_global_position().clamp(
            Vector2::new(0.0, 0.0) + self.sprite_offset,
            screen_size - self.sprite_offset,
        );

        self.body.set_global_position(pos);
    }

    pub fn die(&mut self) {
        self.body.queue_free();
    }
}
