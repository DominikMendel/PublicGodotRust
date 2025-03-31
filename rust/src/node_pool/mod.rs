pub mod node_pool_utils;
mod nodes_pool_container;

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    isize, usize,
};

use godot::{
    classes::{node, INode, Node, ResourceLoader},
    obj::NewAlloc,
    prelude::*,
};
use node_pool_utils::{NodePoolError, Poolable, PoolableState};
use nodes_pool_container::NodesPoolContainer;

// pub enum NodeInfo {
//     Active(Gd<Node>),
//     InactiveAndReady(Gd<Node>),
//     Dead,
// }

#[derive(GodotClass)]
// no_init means can only be made in Rust, no Godot or Editor
// #[class(no_init, base=Node)]
#[class(base=Node)]
pub struct NodePool {
    base: Base<Node>,
    // TODO How to add a "Gd<ParentNode" and when to add it?
    nodes_container: Option<NodesPoolContainer>,
    // size: usize,
    // nodes: Vec<Gd<Node>>,
    // active_nodes: HashSet<usize>,
    // inactive_nodes: HashSet<usize>,
    // dead_nodes: HashSet<usize>,
}

#[godot_api]
impl INode for NodePool {
    fn init(base: Base<Node>) -> Self {
        NodePool {
            base,
            nodes_container: None,
            // size: 0, // TODO Need to add a different init function to set size
            // nodes: Vec::new(),
            // active_nodes: HashSet::new(),
            // inactive_nodes: HashSet::new(),
            // dead_nodes: HashSet::new(),
        }
    }

    fn ready(&mut self) {
        // self.add_all_inactive_nodes_to_tree().unwrap();
        // self.activate_all_ready_nodes().unwrap();

        // let nodes = self.activate_all_ready_nodes().unwrap();
        // godot_print!("Nodes activated are : {:?}", nodes);
        // let active_nodes = self.get_ready_nodes().unwrap();
        // godot_print!("Ready nodes are : {:?}", active_nodes);

        // let node = T::new_alloc();
        // self.nodes.push(node.clone());
        // // TODO might need to ".call deferred"
        // self.base_mut().add_child(&node);

        // let mut resource_loader = ResourceLoader::singleton();
        // let scene: Gd<PackedScene> =
        //     ResourceLoader::load(&mut resource_loader, "res://crystal.tscn")
        //         .unwrap()
        //         .cast();
        // let node: Gd<T> = scene.instantiate_as();
        // self.projectiles.push(node.clone());
        // // BUG this should be added to world node and not player for movement sake
        // self.base_mut()
        //     .get_tree()
        //     .unwrap()
        //     .get_root()
        //     .unwrap()
        //     .call_deferred("add_child", &[node.to_variant()]);
    }

    fn physics_process(&mut self, delta: f64) {}
}

#[godot_api]
impl NodePool {
    // #[func]
    // pub fn new(mut parent_node: Gd<Node>) -> Gd<Self> {
    //     let node = Gd::<NodePool>::from_init_fn(|base| NodePool {
    //         base,
    //         nodes_container: None,
    //     });
    //
    //     parent_node.add_child(&node);
    //
    //     node
    // }

    #[func]
    fn node_state_change_listener(
        &mut self,
        signaling_node: Gd<Node>,
        old_state: PoolableState,
        new_state: PoolableState,
    ) {
        godot_print!(
            // TODO DJM 03/29 this isn't triggering
            "node state change listener. Node {}, old state {:?}, new {:?}",
            signaling_node,
            old_state,
            new_state
        );
        if let Some(container) = self.nodes_container.as_mut() {
            container.set_node_state(signaling_node, old_state, new_state);
        }
    }
}

impl NodePool {
    pub fn initialize_nodes(
        &mut self,
        mut nodes: Vec<DynGd<Node, dyn Poolable>>,
    ) -> Result<(), NodePoolError> {
        if self.nodes_container.is_some() {
            return Err(NodePoolError::NodePoolAlreadyInitialized);
        }

        let callable = &self.base().callable("node_state_change_listener");
        nodes.iter_mut().for_each(|node| {
            // Add all nodes as child to this node
            self.base_mut()
                .call_deferred("add_child", &[node.to_variant()]);

            // Add state change listeners
            node.dyn_bind_mut().add_state_changed_listener(callable);
        });

        self.nodes_container = Some(NodesPoolContainer::new(nodes));
        Ok(())
    }

    // pub fn initialize_nodes<T>(
    //     // pub fn initialize_nodes<T, U>(
    //     &mut self,
    //     mut nodes: Vec<Gd<T>>,
    //     // parent_node: Gd<U>, // Now making it self.node
    // ) -> Result<(), NodePoolError>
    // where
    //     T: GodotClass + Inherits<Node> + NewAlloc + Poolable,
    //     // U: GodotClass + Inherits<Node> + NewAlloc,
    // {
    //     if self.nodes_container.is_some() {
    //         return Err(NodePoolError::NodePoolAlreadyInitialized);
    //     }
    //
    //     // Add the listner to each node
    //     let callable = &self.base().callable("node_state_change_listener");
    //     nodes.iter_mut().for_each(|node| {
    //         // QUESTION: Why can't I do this? add_recyclable_listener is from "Poolable"
    //         // TODO DJM make it a dynGd to call on trait
    //         // node.add_recyclable_listener(callable);
    //
    //         let node = node.clone();
    //         self.base_mut().add_child(&node);
    //         let mut node: Gd<Node> = node.upcast();
    //         node.connect("poolable_state_changed", callable);
    //     });
    //
    //     self.nodes_container = Some(NodesPoolContainer::new(nodes, self.to_gd()));
    //     Ok(())
    // }

    pub fn get_node(
        &self,
        state: PoolableState,
    ) -> Result<DynGd<Node, dyn Poolable>, NodePoolError> {
        if let Some(container) = self.nodes_container.as_ref() {
            return container.get_node(state);
        }

        Err(NodePoolError::NodePoolContainerNotCreated)
    }

    pub fn get_nodes(
        &self,
        state: PoolableState,
    ) -> Result<Vec<DynGd<Node, dyn Poolable>>, NodePoolError> {
        if let Some(container) = self.nodes_container.as_ref() {
            return container.get_nodes(state);
        }

        Err(NodePoolError::NodePoolContainerNotCreated)
    }

    fn set_node_state(
        &mut self,
        node: Gd<Node>,
        old_state: PoolableState,
        new_state: PoolableState,
    ) -> Result<(), NodePoolError> {
        if let Some(container) = self.nodes_container.as_mut() {
            return container.set_node_state(node, old_state, new_state);
        }

        Err(NodePoolError::NodePoolContainerNotCreated)
    }

    //
    // TODO Make result?
    // fn get_active_nodes(&mut self) -> Option<Vec<Gd<Node>>> {
    //     if let Some(ref mut nodes) = self.nodes_container {
    //         return Some(nodes.get_active_nodes());
    //     }
    //
    //     None
    // }

    // fn activate_all_ready_nodes(&mut self) -> Result<Vec<Gd<Node>>, NodePoolError> {
    //     if let Some(ref mut container) = self.nodes_container {
    //         return container.activate_all_ready_nodes();
    //     }
    //
    //     Err(NodePoolError::NodePoolContainerNotCreated)
    // }
    //
    // fn add_all_inactive_nodes_to_tree(&mut self) -> Result<(), NodePoolError> {
    //     if let Some(ref mut container) = self.nodes_container {
    //         return container.add_all_inactive_nodes_to_tree();
    //     }
    //
    //     Err(NodePoolError::NodePoolContainerNotCreated)
    // }
    //
    // pub fn get_active_node(&self) -> Result<Gd<Node>, NodePoolError> {
    //     if let Some(ref container) = self.nodes_container {
    //         return container.get_active_node();
    //     }
    //
    //     Err(NodePoolError::NodePoolContainerNotCreated)
    // }
    //
    // pub fn get_active_nodes(&self) -> Result<Vec<Gd<Node>>, NodePoolError> {
    //     if let Some(ref container) = self.nodes_container {
    //         return container.get_active_nodes();
    //     }
    //
    //     Err(NodePoolError::NodePoolContainerNotCreated)
    // }
    //
    // pub fn get_ready_node(&self) -> Result<Gd<Node>, NodePoolError> {
    //     if let Some(ref container) = self.nodes_container {
    //         return container.get_inactve_and_ready_node();
    //     }
    //
    //     Err(NodePoolError::NodePoolContainerNotCreated)
    // }
    //
    // pub fn get_ready_nodes(&self) -> Result<Vec<Gd<Node>>, NodePoolError> {
    //     if let Some(ref container) = self.nodes_container {
    //         return container.get_inactive_and_ready_nodes();
    //     }
    //
    //     Err(NodePoolError::NodePoolContainerNotCreated)
    // }

    // fn get_inactive_node<T: GodotClass + Inherits<Node> + NewAlloc>(
    //     &mut self,
    // ) -> Result<Gd<T>, NodePoolError> {
    //     // let index = self.inactive_nodes.
    //     // Search inactive nodes first
    //     let index: usize;
    //     if let Some(&inactive_index) = self.inactive_nodes.iter().next() {
    //         index = inactive_index;
    //     } else if let Some(&dead_index) = self.dead_nodes.iter().next() {
    //         index = dead_index;
    //     } else {
    //         return Err(NodePoolError::AllNodesActive);
    //     }
    //
    //     let node = self.nodes.get_mut(index).unwrap().clone();
    //     // TODO Need to cast to type T
    //     let node: Gd<T> = node.cast();
    //     // let node: &mut self.nodes.
    //
    //     Ok(node)
    // }
    //
    // fn create_node<T: GodotClass + Inherits<Node> + NewAlloc>(
    //     &mut self,
    //     item: T,
    // ) -> Result<Gd<Node>, NodePoolError> {
    //     // let node = T::new_alloc();
    //     // // TODO down or up cast to Node type
    //     // // let node: Gd<Node> = node.cast();
    //     // self.nodes.push(node.clone());
    //     // // TODO might need to ".call deferred"
    //     // self.base_mut().add_child(&node);
    //
    //     Err(NodePoolError::NotEnoughNodesInPool)
    // }
    //
    // fn create_nodes<T: GodotClass + Inherits<Node> + NewAlloc>(
    //     &mut self,
    //     items: Vec<T>,
    // ) -> Result<Vec<Gd<Node>>, NodePoolError> {
    //     let mut nodes: Vec<Gd<Node>> = Vec::new();
    //     for item in items {
    //         match self.create_node(item) {
    //             Ok(node) => {
    //                 nodes.push(node);
    //             }
    //             Err(_) => {
    //                 // If any error with 1 node, kill any created nodes
    //                 // Need to manually free Gd nodes or memory leak
    //                 nodes.iter_mut().for_each(|node| {
    //                     node.queue_free();
    //                 });
    //
    //                 // TODO Better error handling?
    //                 return Err(NodePoolError::NotEnoughNodesInPool);
    //             }
    //         }
    //     }
    //
    //     Ok(nodes)
    // }
    //
    // // TODO Return an error code or Result?
    // fn free_node<T: GodotClass + Inherits<Node> + NewAlloc>(&mut self, node: &Gd<T>) {}
    //
    // fn free_nodes<T: GodotClass + Inherits<Node> + NewAlloc>(&mut self, nodes: Vec<&Gd<T>>) {}
}
