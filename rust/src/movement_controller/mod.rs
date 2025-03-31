use godot::classes::InputEventMouse;
use godot::prelude::*;
use godot::{classes::InputEvent, global::Key};
use std::collections::HashMap;

use crate::game_system::GameSystem;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum KeyAction {
    None,
    Up,
    Down,
    Left,
    Right,
    Fire,
}

impl From<Key> for KeyAction {
    fn from(key: Key) -> Self {
        match key {
            Key::UP | Key::W => KeyAction::Up,
            Key::DOWN | Key::S => KeyAction::Down,
            Key::LEFT | Key::A => KeyAction::Left,
            Key::RIGHT | Key::D => KeyAction::Right,
            Key::SPACE => KeyAction::Fire,
            _ => KeyAction::None,
        }
    }
}

// trait ToAction {
//     fn to_action(&self) -> KeyAction;
// }
//
// impl ToAction for Key {
//     fn to_action(&self) -> KeyAction {
//         match *self {
//             Key::UP | Key::W => KeyAction::Up,
//             Key::DOWN | Key::S => KeyAction::Down,
//             Key::LEFT | Key::A => KeyAction::Left,
//             Key::RIGHT | Key::D => KeyAction::Right,
//             _ => KeyAction::None,
//         }
//     }
// }

#[derive(GodotClass)]
// #[class(init, base=Object)]
// #[class(init, base=Node)]
#[class(base=Node)]
pub struct MovementController {
    // base: Base<Object>,
    base: Base<Node>,
    // is_mouse_control: bool,
    keys_to_listen: Vec<Key>,
    pub key_states: HashMap<KeyAction, bool>,
    pub mouse_position: Vector2,
}

impl GameSystem for MovementController {
    const NAME: &'static str = "MovementController";
}

// unsafe impl Bounds for MovementController {
//     type Memory = MemManual;
//     type Declarer = DeclEngine;
// }

// #[derive(GodotClass)]
// #[class(init, base=Object)]
// pub struct MovementController {
//     base: Base<Object>,
//     is_mouse_control: bool,
//     keys: Vec<Key>,
//     key_states: HashMap<Key, bool>,
//     pub mouse_position: Vector2,
// }

#[godot_api]
impl INode for MovementController {
    // fn init(base: Base<Object>) -> Self {
    fn init(base: Base<Node>) -> Self {
        godot_print!("MovementController created!");
        MovementController {
            base,
            // is_mouse_control: true,
            keys_to_listen: vec![
                Key::LEFT,
                Key::RIGHT,
                Key::UP,
                Key::DOWN,
                Key::Q,
                Key::W,
                Key::E,
                Key::R,
                Key::A,
                Key::S,
                Key::D,
                Key::F,
                Key::SPACE,
            ],
            key_states: HashMap::new(),
            mouse_position: Vector2::ZERO,
        }
    }

    fn enter_tree(&mut self) {
        godot_print!("MovementController in tree.");
    }

    // TODO Check if this is in "Object" type
    fn input(&mut self, event: Gd<InputEvent>) {
        // godot_print!("Input from Movement");
        match event.try_cast::<InputEventMouse>() {
            Ok(e) => {
                self.mouse_position = e.get_position();
                // self.base_mut().set_position(mouse_position);
                return;
            }
            Err(_) => {}
        }
        self.keys_to_listen.iter().for_each(|key| {
            let is_key_pressed = Input::singleton().is_key_pressed(*key);
            // godot_print!("is key pressed {}, for key {:?}", is_key_pressed, key);
            // let action = match *key {
            //     Key::UP | Key::W => KeyAction::Up,
            //     Key::DOWN | Key::S => KeyAction::Down,
            //     Key::LEFT | Key::A => KeyAction::Left,
            //     Key::RIGHT | Key::D => KeyAction::Right,
            //     _ => KeyAction::None,
            // };
            self.key_states
                .insert(KeyAction::from(*key), is_key_pressed);
        })
    }
}
