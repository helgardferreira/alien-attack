use crate::enemy::Enemy;
use godot::engine::{Node2D, Node2DVirtual, Timer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct EnemySpawner {
    #[export]
    enabled: bool,

    enemy_scene: Gd<PackedScene>,
    spawn_positions: Option<Gd<Node2D>>,

    #[base]
    node: Base<Node2D>,
}

#[godot_api]
impl Node2DVirtual for EnemySpawner {
    fn init(node: Base<Node2D>) -> Self {
        Self {
            enabled: true,
            enemy_scene: PackedScene::new(),
            spawn_positions: None,
            node,
        }
    }

    fn ready(&mut self) {
        self.enemy_scene = load("res://scenes/enemy.tscn");
        self.spawn_positions = Some(self.node.get_node_as::<Node2D>("SpawnPositions"));

        let mut timer = self.node.get_node_as::<Timer>("Timer");
        timer.connect(
            "timeout".into(),
            Callable::from_object_method(
                self.node.clone().cast::<EnemySpawner>(),
                "on_timer_timeout",
            ),
        );
    }
}

#[godot_api]
impl EnemySpawner {
    #[func]
    fn on_timer_timeout(&mut self) {
        if let Some(position) = self.get_random_spawn() {
            if self.enabled {
                self.spawn_enemy(position);
            }
        }
    }

    fn get_random_spawn(&mut self) -> Option<Vector2> {
        if let Some(spawn_positions) = &mut self.spawn_positions {
            let spawn_children = spawn_positions.get_children();
            if let Some(node) = spawn_children.pick_random() {
                let node = node.cast::<Node2D>();

                Some(node.get_global_position())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn spawn_enemy(&mut self, position: Vector2) {
        let mut enemy = self.enemy_scene.instantiate_as::<Enemy>();

        enemy.set_global_position(position);
        self.node
            .emit_signal("enemy_spawned".into(), &[enemy.to_variant()]);
    }

    #[signal]
    fn enemy_spawned(&mut self, enemy: Gd<Enemy>) {}
}
