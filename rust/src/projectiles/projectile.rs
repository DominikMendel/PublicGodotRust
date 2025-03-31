use godot::prelude::*;

pub trait Projectile {
    fn damage(&self) -> u32 {
        0
    }
    fn fire(&mut self, starting_position: Vector2, direction: Vector2) {}
}

pub enum ProjectileState {
    Dead,
    AliveNotFired,
    AliveFired,
}
