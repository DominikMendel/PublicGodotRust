use godot::{
    builtin::Callable,
    classes::Node,
    obj::{Gd, GodotClass, Inherits, NewAlloc},
    prelude::*,
};

use super::nodes_pool_container::NodesPoolContainer;

#[derive(Clone, Copy, PartialEq, Eq, Hash, GodotConvert, Var, Export, Debug)]
#[godot(via = GString)]
pub enum PoolableState {
    ActiveAndInUse,
    ActiveAndWaiting,
    Deactive,
}

impl PoolableState {
    pub fn iter() -> impl Iterator<Item = PoolableState> {
        vec![Self::ActiveAndInUse, Self::ActiveAndWaiting, Self::Deactive].into_iter()
    }
}

// pub struct PoolableSignal<F> {
//     signal: F,
// }
//
// pub trait Listener {
//     fn add_recyclable_listener<F>(&mut self, func: F)
//     where
//         F: Fn(&mut NodesPoolContainer) -> i32;
// }

/// Don't forget #[godot_dyn] in implementations
// pub trait Poolable: GodotClass + Inherits<Node> + NewAlloc {
pub trait Poolable {
    /// Create a new node of Self, and return an upcast of Node
    fn new_poolable(init_state: PoolableState) -> DynGd<Node, dyn Poolable>
    where
        Self: Sized;
    // fn new_poolable(self, init_state: PoolableState) -> DynGd<Node, dyn Poolable>; // TODO Might not be needed if "GodotClass"
    // fn new_poolable(init_state: PoolableState) -> DynGd<Node, dyn Poolable>; // TODO Might not be needed if "GodotClass"
    fn get_current_state(&self) -> Option<PoolableState>;
    fn add_state_changed_listener(&mut self, callable: &Callable);
    fn remove_state_changed_listener(&mut self, callable: &Callable);
    // fn add_recyclable_listener(&mut self, func: &dyn Fn(i32) -> i32);
    // fn remove_recyclable_listener(&mut self, func: &dyn Fn(i32) -> i32);
}

// pub trait Poolable: GodotClass {
//     fn new(init_state: PoolableState) -> Gd<Self>;
//     fn transition_to_state(&mut self, state: PoolableState);
//     //
//     // // Option 1: Get the singal from the Node, then connect manually
//     // fn get_state_change_signal(&self) -> Signal;
//
//     // Option 2: Send a "Callable" to the Node to add to it's Signal
//     fn add_recyclable_listener(&mut self, callable: &Callable);
//     fn remove_recyclable_listener(&mut self, callable: &Callable);
// }

#[derive(Debug)]
pub enum NodePoolError {
    NoNodeFound,
    ErrorInvalidInput,
    NotEnoughNodesInPool,
    NodePoolAlreadyInitialized,
    AllNodesActive,
    NodesMapNotInitialized,
    NodePoolContainerNotCreated,
    ErrorCreatingNode,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeState {
    Active,
    InactiveAndReady,
    InactiveNeedsTree,
    Dead,
}

impl NodeState {
    pub fn iter() -> impl Iterator<Item = NodeState> {
        vec![
            Self::Active,
            Self::InactiveAndReady,
            Self::InactiveNeedsTree,
            Self::Dead, // TODO This might be the same state as InactiveNeedsTree
        ]
        .into_iter()
    }
}

pub struct NodeInfo {
    pub node: Gd<Node>,
    pub state: NodeState,
}

impl NodeInfo {
    pub fn new<T: Inherits<Node>>(node_to_cast: Gd<T>) -> Self {
        NodeInfo {
            node: node_to_cast.upcast(), // Casts to Gd<Node>
            state: NodeState::InactiveNeedsTree,
        }
    }
}
