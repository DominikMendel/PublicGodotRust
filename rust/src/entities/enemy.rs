use godot::{
    classes::{CharacterBody2D, ICharacterBody2D},
    obj::WithBaseField,
    prelude::*,
};

use crate::{entities::player::Damageable, projectiles::projectile::Projectile};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Enemy {
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Enemy {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Enemy { base }
    }
}

#[godot_dyn]
impl Projectile for Enemy {
    fn damage(&self) -> u32 {
        10
    }
}

#[godot_dyn]
impl Damageable for Enemy {
    fn take_tamage(&mut self, _: u32) {
        // No health, just delete self
        self.base_mut().queue_free();
    }
}

// #[godot_api]
// impl Enemy {
//     #[func]
//     fn deal_damage(&self) -> u32 {
//         10
//     }
// }
