use core::panic;

use godot::classes::resource_loader;
use godot::classes::visual_shader_node_float_parameter::Hint;
use godot::classes::Area2D;
use godot::classes::CharacterBody2D;
use godot::classes::ClassDb;
use godot::classes::ICharacterBody2D;
use godot::classes::InputEvent;
use godot::classes::ResourceLoader;
use godot::prelude::*;
use godot::{global::Key, obj::WithBaseField};

use super::enemy;
use super::enemy::Enemy;
use crate::game_system::GameSystem;
use crate::movement_controller::KeyAction;
use crate::movement_controller::MovementController;
use crate::node_pool;
use crate::node_pool::node_pool_utils::Poolable;
use crate::node_pool::node_pool_utils::PoolableState;
use crate::node_pool::NodePool;
use crate::projectiles::crystal;
use crate::projectiles::crystal::Crystal;
use crate::projectiles::projectile;
use crate::projectiles::projectile::Projectile;

// mod movement_controller;
// use movement_controller::MovementController;
pub trait Damageable {
    fn take_tamage(&mut self, damage: u32);
}

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    base: Base<CharacterBody2D>,
    speed: f64,
    angular_speed: f64,
    health: u32,
    // TODO make this an export to test
    armor: u32,

    // controller: Option<Gd<MovementController>>,
    // #[init(node = "/root/GlobalMovementController")]
    controller: OnReady<Gd<MovementController>>,
    // TODO Make this an array of generic projectiles for selection
    // projectiles: Vec<Gd<Crystal>>,
    projectile_pool: OnReady<Gd<NodePool>>,
}

#[godot_dyn]
impl Damageable for Player {
    fn take_tamage(&mut self, damage: u32) {
        let mut health = self.health;
        if damage >= health {
            health = 0;
        } else {
            health -= damage;
        }

        godot_print!("Damage is taken {}. Health remaining {}", damage, health);
        self.base_mut()
            .emit_signal("hit", &[damage.to_variant(), health.to_variant()]);

        self.health = health;
    }
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Hello world! TEST"); // Prints to the Godot console

        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            health: 100,
            armor: 1,
            // control: MovementController::init(),
            // controller: None,
            // controller: OnReady::manual(),
            controller: OnReady::node("/root/GlobalMovementController"),
            // projectiles: Vec::new(),
            // TODO DJM Should this allow be done in on_ready? This maybe a OnReady type is needed?
            projectile_pool: OnReady::manual(),
            base,
        }
    }

    fn ready(&mut self) {
        // TODO Hard coded value
        // let mut projectile_pool = Vec::new();
        // for i in 0..5 {
        //     projectile_pool.push(Crystal::new_alloc());
        //     // let test = PackedScene::new_gd();
        //     // let test = test.instantiate_as();
        //     // let crystal = Crystal::new_alloc();
        //     // self.projectiles.push(crystal.clone());
        //     // self.base_mut().add_child(&crystal);
        //
        //     let mut resource_loader = ResourceLoader::singleton();
        //     let scene: Gd<PackedScene> =
        //         ResourceLoader::load(&mut resource_loader, "res://crystal.tscn")
        //             .unwrap()
        //             .cast();
        //     let crystal: Gd<Crystal> = scene.instantiate_as();
        //     self.projectiles.push(crystal.clone());
        //     // BUG this should be added to world node and not player for movement sake
        //     self.base_mut()
        //         .get_tree()
        //         .unwrap()
        //         .get_root()
        //         .unwrap()
        //         .call_deferred("add_child", &[crystal.to_variant()]);
        //
        //     // self.base_mut().add_sibling(&crystal); // TODO need to call call_defered?
        //     // self.base_mut()
        //     //     .call_deferred("add_sibling", &[crystal.to_variant()]);
        //     // self.base_mut().add_child(&crystal);
        //
        //     // let mut resource_loader = ResourceLoader::singleton();
        //     // let scene = ResourceLoader::load(&mut resource_loader, "res://crystal.tscn").unwrap();
        //     // let scene: Gd<Crystal> = scene.cast();
        //     // self.projectiles.push(scene);
        //     // self.base_mut().add_child(scene);
        //     // let crystal = load::<Crystal>("res://crystal.tscn");
        //     // let crystal: Gd<Crystal> = load("res://crystal.tscn");
        //     // let node_class = ClassDb::get_class("Crystal").unwrap();
        //     // TODO self add_child(crystal)
        // }

        // let projectile_pool: Vec<Gd<Crystal>> = (0..5).map(|_| Crystal::new_alloc()).collect();
        // let projectile_pool: Vec<Gd<Crystal>> = (0..5)
        //     .map(|_| {
        //         let crystal = Crystal::new_alloc();
        //         let crystal = crystal.into_dyn::<dyn Poolable>();
        //     })
        //     .collect();

        // let mut resource_loader = ResourceLoader::singleton();
        // let scene = ResourceLoader::load(&mut resource_loader, "res://crystal.tscn").unwrap();

        let projectile_pool: Vec<DynGd<Node, dyn Poolable>> = (0..5)
            .map(|_| Crystal::new_poolable(PoolableState::ActiveAndWaiting))
            .collect();

        let mut tree = self.base_mut().get_tree().unwrap().get_root().unwrap();
        let mut pool = NodePool::new_alloc();
        // self.base_mut().add_child(&pool);
        tree.call_deferred("add_child", &[pool.to_variant()]);
        // TODO handle error
        let _ = pool.bind_mut().initialize_nodes(projectile_pool);
        self.projectile_pool.init(pool);

        // let _ = self
        //     .projectile_pool
        //     .bind_mut()
        //     .initialize_nodes(projectile_pool);

        // self.controller = self.base_mut().try_get_node_as("/root/GlobalMovementController");
        // let controller = self.base_mut().try_get_node_as("/root/GlobalMovementController")
        // self.controller.init(self.base_mut().try_get_node_as("/root/GlobalMovementController"));
        // let controller: Option<Gd<MovementController>> = self.base_mut().try_get_node_as("/root/GlobalMovementController");
        // match controller {
        // // match self.base_mut().try_get_node_as("/root/GlobalMovementController").cast::<MovementController> {
        //     Some(x) => {
        //         // self.controller = controller;
        //         self.controller = Some(x);
        //     }
        //     None => {
        //         godot_error!("MovementController could not be found!");
        //         panic!();
        //     }
        // };

        // self.controller = self.base_mut().try_get_node_as("/root/GlobalMovementController");
        // self.controller = self.base_mut().try_get_node_as("/root/GlobalMovementController") else {
        //     godot_error!("MovementController could not be found!");
        //     panic!();
        // };
        // else {
        // // let controller = Some(self.base_mut().get_tree().unwrap().get_root().unwrap().get_node_as("GlobalMovementController")) else {
        //     godot_error!("MovementController could not be found!");
        //     panic!();
        // };
        // self.controller = controller;
    }

    fn process(&mut self, delta: f64) {
        for (&key, &value) in &self.controller.bind().key_states {
            match (key, value) {
                // TODO hardcoded to just 1
                // TODO Should this fire be done in physics_process instead?
                (KeyAction::Fire, true) => {
                    let mut mouse_position = self.base().get_local_mouse_position();
                    if mouse_position.length() > 0.0 {
                        mouse_position = mouse_position.normalized();
                    }

                    let global_position = self.base().get_global_position();
                    // let projectile = self.projectiles.get_mut(0).unwrap();
                    if let Ok(projectile) = self
                        .projectile_pool
                        .bind()
                        .get_node(PoolableState::ActiveAndWaiting)
                    {
                        let variant = projectile.to_variant();
                        let mut projectile: DynGd<Node, dyn Projectile> = variant.to();
                        projectile
                            .dyn_bind_mut()
                            .fire(global_position, mouse_position);
                    } else {
                        godot_error!("No projectile received for user fire in Player node.");
                    }
                }
                _ => {}
            }
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        // Working:
        // let mouse_position = self.controller.bind().mouse_position;
        // self.base_mut().set_position(mouse_position);

        // Old code for test

        // Mouse events
        // match event.try_cast()::<InputEventMouse>() {
        // }

        // let controller = godot::classes::Engine::singleton()
        //     .get_singleton(&StringName::from("MovementController"))
        //     .unwrap().cast::<MovementController>();
        // let controller: Gd<MovementController> = godot::classes::Engine::singleton()
        //     .get_singleton(&StringName::from("MovementController"))
        //     .unwrap().cast();
        // controller.bind().
        // controller.get_class();
        // let singletons = Engine::singleton().get_singleton_list();
        // for i in 0..singletons.len() {
        //     godot_print!("Singleton: {}", singletons[i]);
        // }

        // let controller: Gd<MovementController> = self.base_mut().get_tree().unwrap().get_root().unwrap().get_node_as("GlobalMovementController");
        // let controller: Gd<MovementController> = self.base_mut().get_tree().get_root().get_node_as("GlobalMovementController");

        // let controller = MovementController::singleton();
        // let controller = &self.controller;
        // let controller = self.controller.as_ref();
        // let controller = controller.unwrap();
        // let controller = controller.bind();
        // if let controller = self.controller.as_ref().unwrap(){
        //     let mouse_position = controller.bind().mouse_position;
        //     self.base_mut().set_position(mouse_position);
        // }

        // // let mut mouse_position = Vector2::new(0.0, 0.0);
        // let mouse_position = self.controller.as_ref().map(|ctrl| ctrl.bind().mouse_position).unwrap();
        // // let controller = self.controller.as_ref().unwrap().bind();
        // // let mouse_position = controller.mouse_position;
        // // drop(controller);
        // // self.base_mut().set_position(mouse_position);

        // // drop(controller);
        // self.base_mut().set_position(mouse_position);

        // TODO Is this necessary to explicitly do a "let" followed by a drop?
        // let controller = &mut self.base_mut().control;
        // self.control.process_input(event);
        // // TODO Probably a better way to do this? i.e., put control logic in control
        // // Or somehow just cleanly passthrough input
        // let position = self.control.mouse_position;
        // self.base_mut().set_position(position);
        // Can't do this
        // self.base_mut().set_position(self.control.mouse_position);
    }

    fn physics_process(&mut self, delta: f64) {
        // In GDScript, this would be:
        // rotation += angular_speed * delta

        let mut current_position = self.base_mut().get_position();
        for (&key, &value) in &self.controller.bind().key_states {
            match (key, value) {
                // TODO normalize diagnals
                (KeyAction::Up, true) => current_position.y -= 1.00 * 1000.0 * delta as f32,
                (KeyAction::Down, true) => current_position.y += 1.00 * 1000.0 * delta as f32,
                (KeyAction::Left, true) => current_position.x -= 1.00 * 1000.0 * delta as f32,
                (KeyAction::Right, true) => current_position.x += 1.00 * 1000.0 * delta as f32,
                _ => {}
            }
        }
        self.base_mut().set_position(current_position);
        // self.controller
        //     .bind()
        //     .key_states
        //     .iter()
        //     .map(|(&key, &value)| match key {
        //         Key::UP => current_position.y -= 1.00 * 1000.0 * delta as f32,
        //         _ => {}
        //     });

        // let radians = (self.angular_speed * delta) as f32;
        // self.base_mut().rotate(radians);
        //
        // let rotation = self.base().get_rotation();
        // let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
        // self.base_mut().translate(velocity * delta as f32);
        // godot_print!("From Physics: Speed has been increased.");
        // self.increase_speed(10.0);
        // The 'rotate' method requires a f32,
        // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32
    }
}

#[godot_api]
impl Player {
    #[func]
    fn increase_speed(&mut self, amount: f64) {
        self.speed += amount;
        self.base_mut().emit_signal("speed_increased", &[]);
    }

    #[func]
    fn button_pressed(&mut self) {
        let visible = self.base().is_visible();
        self.base_mut().set_visible(!visible);
    }

    #[func]
    fn on_body_entered(&mut self, item: Gd<Area2D>) {
        let area_id = item.instance_id();
        godot_print!("Body entered by {}", area_id);
        if let Some(parent) = item.get_parent() {
            let variant = parent.to_variant();
            if let Ok(enemy) = variant.try_to::<DynGd<Node, dyn Projectile>>() {
                let damage = enemy.dyn_bind().damage();
                self.take_tamage(damage);
            }

            if let Ok(mut damageable) = variant.try_to::<DynGd<Node, dyn Damageable>>() {
                damageable.dyn_bind_mut().take_tamage(self.armor);
            }
        }
    }

    #[signal]
    fn hit(&self, damage_taken: u32, remaining_health: u32) {}

    #[signal]
    fn dead() {}

    #[signal]
    fn speed_increased() {
        godot_print!("Speed has been increased.");
    }
}
