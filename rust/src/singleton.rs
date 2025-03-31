use godot::prelude::*;

use crate::game_system::GameSystem;

// #[class(init, base=Object)]
#[derive(GodotClass)]
#[class(base=Object)]
struct MySingleton {
    base: Base<Object>,
}

impl GameSystem for MySingleton  {
    const NAME: &'static str = "MySingleton";
}


#[godot_api]
impl IObject for MySingleton {
    fn init(base: Base<Object>) -> Self {
        godot_print!("MySingleton created!");
        MySingleton {
            base,
        }
    }
}
