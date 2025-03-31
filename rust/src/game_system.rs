use godot::{classes::Engine, obj::{bounds, Bounds, NewAlloc}, prelude::*};

pub trait GameSystem:
    GodotClass + Bounds<Declarer = bounds::DeclUser> + NewAlloc + Inherits<Object>
{
    const NAME: &'static str;

    fn singleton() -> Gd<Self> {
        Engine::singleton()
            .get_singleton(Self::NAME)
            .unwrap() // THIS PANICS
            .cast::<Self>()
    }
    fn initialize() -> Gd<Self> {
        let game_system = Self::new_alloc();
        Engine::singleton().register_singleton(Self::NAME, &game_system);
        game_system
    }

    fn exit(&mut self) {
        Engine::singleton().unregister_singleton(Self::NAME);
    }
    #[allow(unused_variables)]
    fn physics_process(&mut self, delta: f64) {}
    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {}
}
