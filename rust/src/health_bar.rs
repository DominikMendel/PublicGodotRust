use godot::{
    classes::{IProgressBar, ProgressBar},
    obj::WithBaseField,
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=ProgressBar)]
struct HealthBar {
    base: Base<ProgressBar>,
}

#[godot_api]
impl IProgressBar for HealthBar {
    fn init(base: Base<ProgressBar>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        // self.base_mut().set("max_value", &100.to_variant());
        // self.base_mut().set("value", &100.to_variant());

        self.base_mut().set_max(100.0);
        self.base_mut().set_value(100.0);
    }
}

#[godot_api]
impl HealthBar {
    #[func]
    fn on_player_hit(&mut self, _damage_taken: u32, remaining_health: u32) {
        // self.base_mut().set("value", &remaining_health.to_variant());
        self.base_mut().set_value(remaining_health as f64);
    }
}
