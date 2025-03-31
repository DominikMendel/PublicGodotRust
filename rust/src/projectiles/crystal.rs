use godot::{
    classes::{CharacterBody2D, ICharacterBody2D, ResourceLoader, Timer},
    prelude::*,
};

use crate::node_pool::node_pool_utils::{Poolable, PoolableState};

use super::projectile::{self, Projectile, ProjectileState};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Crystal {
    base: Base<CharacterBody2D>,
    state: Option<PoolableState>,
    speed: f32,
    trajectory: Vector2,
    timeout: Gd<Timer>,
}

#[godot_api]
impl ICharacterBody2D for Crystal {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Crystal {
            base,
            state: None,
            speed: 100.0,
            trajectory: Vector2::ZERO,
            timeout: Timer::new_alloc(),
        }
    }

    fn ready(&mut self) {
        let timeout = self.timeout.clone();
        self.base_mut().add_child(&timeout);
        let callable = &self.base().callable("timeout_recieved");
        self.timeout.connect("timeout", callable);
        self.timeout.set_wait_time(5.0); // TODO hard coded

        if let Some(state) = self.state {
            self.apply_state(state);
        } else {
            godot_error!("Inner Poolable state not set in on_ready!");
        }
    }

    fn physics_process(&mut self, delta: f64) {
        // TODO Should this be somewhere else
        if !self.base().is_processing() {
            return;
        }

        match self.state {
            Some(PoolableState::ActiveAndInUse) => {
                self.base_mut().move_and_slide();
            }
            Some(PoolableState::ActiveAndWaiting) => {}
            Some(PoolableState::Deactive) => {}
            None => todo!(),
        }
    }
}

#[godot_dyn]
impl Projectile for Crystal {
    fn damage(&self) -> u32 {
        20
    }

    fn fire(&mut self, starting_position: Vector2, direction: Vector2) {
        match self.state {
            Some(PoolableState::Deactive) => {
                godot_error!("Projectile is dead, cannot fire.");
            }
            Some(PoolableState::ActiveAndInUse) => {
                godot_error!("Projectile is already fired, cannot fire.");
            }
            Some(PoolableState::ActiveAndWaiting) => {
                godot_print!("Firing Crystal in direction : {}", direction);
                self.apply_state(PoolableState::ActiveAndInUse);
                let velocity = direction * self.speed;
                self.base_mut().set_velocity(velocity);
                self.base_mut().set_rotation(direction.angle());
                self.base_mut().set_global_position(starting_position);
            }
            None => todo!(),
        }
    }
}

#[godot_dyn]
impl Poolable for Crystal {
    // fn transition_to_state(&mut self, state: PoolableState) {
    //     let current_state = self.state;
    //     match (state, current_state) {
    //         (PoolableState::ActiveAndInUse, PoolableState::ActiveAndInUse) => return,
    //         (PoolableState::ActiveAndInUse, PoolableState::ActiveAndWaiting) => {
    //             panic!("Cannot make active node inactive.")
    //         }
    //         (PoolableState::ActiveAndInUse, PoolableState::Deactive) => {
    //             panic!("Cannot deactivate currently in use node.")
    //         }
    //         (PoolableState::ActiveAndWaiting, PoolableState::ActiveAndInUse) => {
    //             panic!("Cannot move from Deactive to ActiveInUse. Only in Fire to put in use.")
    //         }
    //         (PoolableState::ActiveAndWaiting, PoolableState::ActiveAndWaiting) => return,
    //         (PoolableState::ActiveAndWaiting, PoolableState::Deactive) => {}
    //         (PoolableState::Deactive, PoolableState::ActiveAndInUse) => {
    //             panic!("Cannot move from Deactive to ActiveInUse. Only in Fire to put in use.")
    //         }
    //         (PoolableState::Deactive, PoolableState::ActiveAndWaiting) => {}
    //         (PoolableState::Deactive, PoolableState::Deactive) => return,
    //     }
    //     self.apply_state(state);
    // }

    fn new_poolable(init_state: PoolableState) -> DynGd<Node, dyn Poolable> {
        let mut resource_loader = ResourceLoader::singleton();
        // TODO DJM hard coded, make it more flexible
        // Like adding an Option<Path> and use this as a default
        let scene: Gd<PackedScene> =
            ResourceLoader::load(&mut resource_loader, "res://crystal.tscn")
                .unwrap()
                .cast();
        let mut node: Gd<Self> = scene.instantiate_as();
        node.bind_mut().state = Some(init_state);

        // let node = Gd::<Crystal>::from_init_fn(|base| Crystal {
        //     base,
        //     state: init_state,
        //     speed: 100.0,
        //     trajectory: Vector2::ZERO,
        //     timeout: Timer::new_alloc(),
        //     // ..self
        // });
        // TODO This upcast might not be needed, but need to test without
        let node = node.upcast::<Node>();
        let node = node.to_variant();
        let node = node.to::<DynGd<Node, dyn Poolable>>();
        return node;
    }

    fn get_current_state(&self) -> Option<PoolableState> {
        self.state
    }

    fn add_state_changed_listener(&mut self, callable: &Callable) {
        if !self.base().is_connected("poolable_state_changed", callable) {
            self.base_mut().connect("poolable_state_changed", callable);
        }
    }

    fn remove_state_changed_listener(&mut self, callable: &Callable) {
        if self.base().is_connected("poolable_state_changed", callable) {
            self.base_mut()
                // self.base_mut().connect("poolable_state_changed", callable.bind(varargs));
                .disconnect("poolable_state_changed", callable);
        }
    }
}

#[godot_api]
impl Crystal {
    fn apply_state(&mut self, state: PoolableState) {
        if self.state.is_none() {
            // Poolable state has never been set, this is a bad state
            godot_warn!("Poolable state has never been set, this is a bad state");
            return;
        }

        match state {
            PoolableState::ActiveAndInUse => {
                self.base_mut().set_visible(true);
                self.base_mut().set_process(true);
                if self.timeout.is_stopped() {
                    self.timeout.start();
                }
            }
            PoolableState::ActiveAndWaiting => {
                self.base_mut().set_visible(true);
                self.base_mut().set_process(true);
            }
            PoolableState::Deactive => {
                self.base_mut().set_visible(false);
                self.base_mut().set_process(false);
                self.timeout.stop();
                self.base_mut().emit_signal("ready_for_recycle", &[]);
            }
        }

        // self.state is Some at this point
        if self.state.unwrap() != state {
            let old_state = self.state.replace(state);
            let self_variant = self.base().to_variant();
            // let self_variant = self.base().clone().to_variant();
            self.base_mut().emit_signal(
                "poolable_state_changed",
                &[
                    self_variant,
                    old_state.unwrap().to_variant(),
                    state.to_variant(),
                ],
            );
        }
    }

    #[signal]
    fn poolable_state_changed(
        // Signaling node is self as node
        signaling_node: Gd<Node>,
        old_state: PoolableState,
        new_state: PoolableState,
    );

    #[signal]
    // TODO DJM 03/30 is this still needed?
    fn ready_for_recycle();

    #[func]
    fn timeout_recieved(&mut self) {
        godot_print!("Crystal node timeout received.");
        self.apply_state(PoolableState::Deactive);
    }
}
