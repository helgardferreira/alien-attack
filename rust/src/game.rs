use crate::enemy::Enemy;
use crate::enemy_spawner::EnemySpawner;
use crate::game_over_screen::GameOverScreen;
use crate::hud::Hud;
use crate::player::Player;
use godot::engine::{display_server::WindowMode, Area2D, DisplayServer, Node2D, Node2DVirtual};
use godot::engine::{AudioStreamPlayer2D, CanvasLayer};
use godot::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Game {
    #[export]
    score: u32,
    #[export]
    lives: i8,
    #[export]
    game_over_delay: u32,

    game_over_sender: Option<mpsc::Sender<()>>,
    game_over_receiver: Option<mpsc::Receiver<()>>,

    game_over_scene: Gd<PackedScene>,
    player: Option<Gd<Player>>,
    ui: Option<Gd<CanvasLayer>>,
    hud: Option<Gd<Hud>>,
    enemy_hit_sound: Option<Gd<AudioStreamPlayer2D>>,
    player_damage_sound: Option<Gd<AudioStreamPlayer2D>>,

    #[base]
    node: Base<Node2D>,
}

#[godot_api]
impl Node2DVirtual for Game {
    fn init(node: Base<Node2D>) -> Self {
        Self {
            score: 0,
            lives: 3,
            game_over_delay: 1200,

            game_over_sender: None,
            game_over_receiver: None,

            game_over_scene: PackedScene::new(),
            player: None,
            ui: None,
            hud: None,
            enemy_hit_sound: None,
            player_damage_sound: None,

            node,
        }
    }

    fn ready(&mut self) {
        self.player = Some(self.node.get_node_as::<Player>("Player"));

        let mut hud = self.node.get_node_as::<Hud>("UI/Hud");
        hud.bind_mut().set_score_label(self.score);
        hud.bind_mut().set_lives_label(self.lives);
        self.hud = Some(hud);

        self.ui = Some(self.node.get_node_as("UI"));

        let mut enemy_spawner = self.node.get_node_as::<EnemySpawner>("EnemySpawner");
        let game = self.node.clone().cast::<Game>();
        enemy_spawner.connect(
            "enemy_spawned".into(),
            Callable::from_object_method(game, "on_enemy_spawned"),
        );

        let mut deathzone = self.node.get_node_as::<Area2D>("Deathzone");
        let game = self.node.clone().cast::<Game>();
        deathzone.connect(
            "area_entered".into(),
            Callable::from_object_method(game, "on_deathzone_entered"),
        );

        self.enemy_hit_sound = Some(self.node.get_node_as("EnemyHitSound"));
        self.player_damage_sound = Some(self.node.get_node_as("PlayerDamageSound"));

        self.game_over_scene = load("res://scenes/game_over_screen.tscn");

        // Experimenting with threads - this could just be done with the godot Timer struct
        // Create an mpsc channel
        let (thread_sender, receiver) = mpsc::channel();
        let (sender, thread_receiver) = mpsc::channel();
        self.game_over_receiver = Some(receiver);
        self.game_over_sender = Some(sender);

        let delay = self.game_over_delay as u64;

        // Setup game over delay timeout thread
        thread::spawn(move || loop {
            thread_receiver.recv().unwrap();
            thread::sleep(Duration::from_millis(delay));
            thread_sender.send(()).unwrap();
        });
    }

    fn process(&mut self, _delta: f64) {
        let toggle_fullscreen =
            Input::singleton().is_action_just_released("fullscreen_toggle".into());

        if toggle_fullscreen {
            let mut ds = DisplayServer::singleton();
            let mode = ds.window_get_mode();

            if mode == WindowMode::WINDOW_MODE_FULLSCREEN {
                ds.window_set_mode(WindowMode::WINDOW_MODE_WINDOWED);
            } else {
                ds.window_set_mode(WindowMode::WINDOW_MODE_FULLSCREEN);
            }
        }

        // If there is a game over receiver and a message is received then display the game over
        // screen
        if let Some(receiver) = &mut self.game_over_receiver {
            if receiver.try_recv().is_ok() {
                self.display_game_over_screen();
            }
        }
    }
}

#[godot_api]
impl Game {
    #[func]
    fn on_deathzone_entered(&mut self, node: Gd<Node2D>) {
        if let Some(mut enemy) = node.clone().try_cast::<Enemy>() {
            enemy.bind_mut().free();
        }
    }

    #[func]
    fn on_enemy_attack(&mut self) {
        self.lives -= 1;

        if let Some(player_damage_sound) = &mut self.player_damage_sound {
            player_damage_sound.play();
        }

        if let Some(hud) = &mut self.hud {
            hud.bind_mut().set_lives_label(self.lives);
        }

        if self.lives <= 0 {
            self.game_over();
        }
    }

    #[func]
    fn on_enemy_died(&mut self) {
        self.score += 100;

        if let Some(hud) = &mut self.hud {
            hud.bind_mut().set_score_label(self.score);
        }

        if let Some(enemy_hit_sound) = &mut self.enemy_hit_sound {
            enemy_hit_sound.play();
        }
    }

    #[func]
    fn on_enemy_spawned(&mut self, mut enemy: Gd<Enemy>) {
        let game = self.node.clone().cast::<Game>();
        enemy.connect(
            "attack".into(),
            Callable::from_object_method(game, "on_enemy_attack"),
        );

        let game = self.node.clone().cast::<Game>();
        enemy.connect(
            "died".into(),
            Callable::from_object_method(game, "on_enemy_died"),
        );

        self.node.add_child(enemy.upcast());
    }

    fn game_over(&mut self) {
        if let Some(player) = &mut self.player {
            player.bind_mut().die();
        }

        self.start_game_over_timer();
    }

    fn start_game_over_timer(&mut self) {
        if let Some(sender) = &mut self.game_over_sender {
            sender.send(()).unwrap();
        }
    }

    fn display_game_over_screen(&mut self) {
        if let Some(ui) = &mut self.ui {
            let mut game_over_screen = self.game_over_scene.instantiate_as::<GameOverScreen>();
            ui.add_child(game_over_screen.clone().upcast());
            game_over_screen.bind_mut().set_score(self.score);
        }
    }
}
