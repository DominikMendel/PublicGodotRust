use std::{
    collections::{hash_map, HashMap, HashSet},
    hash::Hash,
    isize, usize,
};

use godot::{
    classes::{node, INode, Node, ResourceLoader},
    obj::NewAlloc,
    prelude::*,
};

use super::node_pool_utils::{NodeInfo, NodePoolError, NodeState, Poolable, PoolableState};

pub struct NodesPoolContainer {
    /// Used to track the container size to ensure the
    /// nodes doesn't allocate more than what it was init with
    size: usize,
    // nodes: HashMap<NodeState, Vec<NodeInfo>>,
    // nodes: HashMap<PoolableState, Vec<Gd<Node>>>,
    nodes: HashMap<PoolableState, Vec<DynGd<Node, dyn Poolable>>>,
    // parent_node: Gd<Node>,
}

impl NodesPoolContainer {
    // TODO DJM Can I do this with DynGd<T>
    // pub fn new<T, U>(nodes: Vec<DynGd<T, dyn Poolable>>, parent_node: Gd<U>) -> Self
    // where
    //     T: GodotClass + Inherits<Node> + NewAlloc + Poolable,
    //     // T: Poolable,
    //     U: GodotClass + Inherits<Node> + NewAlloc,

    // pub fn new<T, U>(nodes: Vec<DynGd<T, dyn Poolable>>, parent_node: Gd<U>) -> Self
    pub fn new(nodes: Vec<DynGd<Node, dyn Poolable>>) -> Self
// where
    //     T: GodotClass + Inherits<Node> + NewAlloc + Poolable,
    //     // T: GodotClass,
    //     U: GodotClass + Inherits<Node> + NewAlloc,
    {
        let mut hash_map: HashMap<PoolableState, Vec<DynGd<Node, dyn Poolable>>> = HashMap::new();

        for state in PoolableState::iter() {
            hash_map.insert(state, Vec::new());
        }

        let size = nodes.len();

        nodes.into_iter().for_each(|node| {
            let state = node.dyn_bind().get_current_state().expect(
                "Poolable trait being used by Pool Container must have a state at this point.",
            );
            hash_map.get_mut(&state).unwrap().push(node);
        });

        // let parent_node: Gd<Node> = parent_node.upcast();
        // hash_map
        //     .get_mut(&PoolableState::ActiveAndWaiting)
        //     .unwrap()
        //     .extend(nodes.into_iter().map(|node| node.upcast::<Node>()));

        // Put input nodes into hashmap
        // hash_map
        //     .get_mut(&PoolableState::ActiveAndWaiting)
        //     .unwrap()
        //     .extend(nodes.into_iter().map(|node| {
        //         // Add node to tree
        //         let node = node.new_poolable(PoolableState::ActiveAndWaiting);
        //         if let Some(mut parent) = node.get_parent() {
        //             parent.call_deferred("add_child", &[node.to_variant()]);
        //         }
        //
        //         node
        //
        //         // node.into_dyn::<Poolable>()
        //
        //         // Cast to Node to be storable in non-T type.
        //         // Can be casted back at any time by NodePool caller
        //         // return node.upcast::<Node>().clone();
        //     }));

        // hash_map
        //     .get_mut(&PoolableState::ActiveAndWaiting)
        //     .unwrap()
        //     .iter()
        //     .for_each(|node| node.connect_ex);

        NodesPoolContainer {
            size,
            nodes: hash_map,
            // parent_node,
        }

        //
        // nodes.iter_mut().for_each(|node| {
        //     container.add_node(NodeInfo::new(node.clone()));
        // });

        // container
    }

    // pub fn new<T, U>(nodes: Vec<T>, parent_node: Gd<U>) -> Self
    // where
    //     T: GodotClass + Inherits<Node> + NewAlloc + Poolable,
    //     U: GodotClass + Inherits<Node> + NewAlloc,
    // {
    //     // let mut hash_map: HashMap<NodeState, Vec<NodeInfo>> = HashMap::new();
    //
    //     let mut hash_map: HashMap<PoolableState, Vec<Gd<Node>>> = HashMap::new();
    //     for state in PoolableState::iter() {
    //         hash_map.insert(state, Vec::new());
    //     }
    //
    //     let size = nodes.len();
    //     let mut parent_node: Gd<Node> = parent_node.upcast();
    //
    //     // Put input nodes into hashmap
    //     hash_map
    //         .get_mut(&PoolableState::ActiveAndWaiting)
    //         .unwrap()
    //         .extend(nodes.into_iter().map(|node| {
    //             // Add node to tree
    //             parent_node.call_deferred("add_child", &[node.to_variant()]);
    //
    //             // Cast to Node to be storable in non-T type.
    //             // Can be casted back at any time by NodePool caller
    //             return node.upcast::<Node>().clone();
    //         }));
    //
    //     // hash_map
    //     //     .get_mut(&PoolableState::ActiveAndWaiting)
    //     //     .unwrap()
    //     //     .iter()
    //     //     .for_each(|node| node.connect_ex);
    //
    //     NodesPoolContainer {
    //         size,
    //         nodes: hash_map,
    //         parent_node,
    //     }
    //
    //     //
    //     // nodes.iter_mut().for_each(|node| {
    //     //     container.add_node(NodeInfo::new(node.clone()));
    //     // });
    //
    //     // container
    // }

    // fn add_node(&mut self, node: NodeInfo) {
    //     let state = node.state;
    //     self.nodes.get_mut(&state).unwrap().push(node);
    // }

    /// Adds all nodes in InactiveNeedsTree to Godot Tree and updates state + HashMap
    // pub fn add_all_inactive_nodes_to_tree(&mut self) -> Result<(), NodePoolError> {
    //     let nodes_to_add = std::mem::replace(
    //         self.nodes
    //             .get_mut(&NodeState::InactiveNeedsTree)
    //             .ok_or(NodePoolError::NodePoolAlreadyInitialized)?,
    //         vec![],
    //     );
    //
    //     let ready_nodes = self
    //         .nodes
    //         .get_mut(&NodeState::InactiveAndReady)
    //         .ok_or(NodePoolError::NodePoolAlreadyInitialized)?;
    //
    //     ready_nodes.extend(nodes_to_add.into_iter().map(|mut node_info| {
    //         // Add node to tree
    //         self.parent_node
    //             .call_deferred("add_child", &[node_info.node.to_variant()]);
    //
    //         // Update state
    //         node_info.state = NodeState::InactiveAndReady;
    //         node_info
    //     }));
    //
    //     Ok(())
    // }

    // TODO DJM This might need to be removed in future iteration.
    // Container shouldn't move from inactiveAndReady to Active
    // pub fn activate_all_ready_nodes(&mut self) -> Result<Vec<Gd<Node>>, NodePoolError> {
    //     // Remove all nodes in InactiveAndReady state
    //     let nodes_to_move = std::mem::replace(
    //         self.nodes
    //             .get_mut(&NodeState::InactiveAndReady)
    //             .ok_or(NodePoolError::AllNodesActive)?,
    //         vec![], // Same as Vec:new()
    //     );
    //
    //     let active_nodes = self
    //         .nodes
    //         .get_mut(&NodeState::Active)
    //         .ok_or(NodePoolError::AllNodesActive)?;
    //
    //     // Append the nodes and change state
    //     active_nodes.extend(nodes_to_move.into_iter().map(|mut node_info| {
    //         node_info.state = NodeState::Active;
    //         node_info
    //     }));
    //
    //     // TODO This might not handle juggling nodes from Inactive to Active and "Active and Done"
    //     // vs "Active and not done". This returns ALL active nodes, regardless of their active
    //     // state
    //     // let copied_nodes: Vec<Gd<Node>> =
    //     //     active_nodes.iter().map(|node| node.node.clone()).collect();
    //     Ok(active_nodes
    //         .iter()
    //         .map(|node_info| node_info.node.clone())
    //         .collect())
    // }

    pub fn get_nodes(
        &self,
        state: PoolableState,
    ) -> Result<Vec<DynGd<Node, dyn Poolable>>, NodePoolError> {
        if let Some(nodes) = self.nodes.get(&state) {
            return Ok(nodes
                .iter()
                .map(|node| node.clone())
                .collect::<Vec<DynGd<Node, dyn Poolable>>>());
        }

        Err(NodePoolError::NodesMapNotInitialized)
    }

    pub fn get_node(
        &self,
        state: PoolableState,
    ) -> Result<DynGd<Node, dyn Poolable>, NodePoolError> {
        if let Some(nodes) = self.nodes.get(&state) {
            return nodes
                .last()
                .ok_or(NodePoolError::AllNodesActive)
                .map(|node| node.clone());
        }

        Err(NodePoolError::NodesMapNotInitialized)
    }

    // pub fn get_nodes(&self, state: PoolableState) -> Result<Vec<Gd<Node>>, NodePoolError> {
    //     if let Some(nodes) = self.nodes.get(&state) {
    //         return Ok(nodes
    //             .iter()
    //             .map(|node| node.clone())
    //             .collect::<Vec<Gd<Node>>>());
    //     }
    //
    //     Err(NodePoolError::NodesMapNotInitialized)
    //
    //     // let nodes = self
    //     //     .nodes
    //     //     .get(&state)
    //     //     .ok_or(NodePoolError::NodesMapNotInitialized)
    //     //     // .unwrap_or(Err(NodePoolError::NodesMapNotInitialized))
    //     //     // .unwrap()
    //     //     .iter()
    //     //     .map(|node| node.clone())
    //     //     .collect::<Vec<Gd<Node>>>();
    //
    //     // Ok(self
    //     //     .nodes
    //     //     .get(&state)
    //     //     .unwrap_or(Err(NodePoolError::NodesMapNotInitialized))
    //     //     // .unwrap()
    //     //     .iter()
    //     //     .map(|node| node.clone())
    //     //     .collect::<Vec<Gd<Node>>>())
    // }
    //
    // pub fn get_node(&self, state: PoolableState) -> Result<Gd<Node>, NodePoolError> {
    //     if let Some(nodes) = self.nodes.get(&state) {
    //         return nodes
    //             .last()
    //             .ok_or(NodePoolError::AllNodesActive)
    //             .map(|node| node.clone());
    //     }
    //
    //     Err(NodePoolError::NodesMapNotInitialized)
    // }
    //
    // pub fn get_active_node(&self) -> Result<Gd<Node>, NodePoolError> {
    //     if let Some(active_nodes) = self.nodes.get(&NodeState::Active) {
    //         if let Some(node_info) = &active_nodes.get(0) {
    //             return Ok(node_info.node.clone());
    //         }
    //     }
    //
    //     Err(NodePoolError::NodesMapNotInitialized)
    // }
    //
    // pub fn get_active_nodes(&self) -> Result<Vec<Gd<Node>>, NodePoolError> {
    //     if let Some(active_nodes) = self.nodes.get(&NodeState::Active) {
    //         let collected_nodes = active_nodes
    //             .iter()
    //             .map(|node_info| node_info.node.clone())
    //             .collect::<Vec<Gd<Node>>>();
    //         if collected_nodes.is_empty() {
    //             return Err(NodePoolError::AllNodesActive);
    //         }
    //
    //         return Ok(collected_nodes);
    //     }
    //
    //     Err(NodePoolError::NodesMapNotInitialized)
    // }
    //
    // pub fn get_inactve_and_ready_node(&self) -> Result<Gd<Node>, NodePoolError> {
    //     if let Some(active_nodes) = self.nodes.get(&NodeState::InactiveAndReady) {
    //         if let Some(node_info) = &active_nodes.get(0) {
    //             return Ok(node_info.node.clone());
    //         }
    //     }
    //
    //     Err(NodePoolError::NodesMapNotInitialized)
    // }
    //
    // pub fn get_inactive_and_ready_nodes(&self) -> Result<Vec<Gd<Node>>, NodePoolError> {
    //     if let Some(active_nodes) = self.nodes.get(&NodeState::InactiveAndReady) {
    //         let collected_nodes = active_nodes
    //             .iter()
    //             .map(|node_info| node_info.node.clone())
    //             .collect::<Vec<Gd<Node>>>();
    //         if collected_nodes.is_empty() {
    //             return Err(NodePoolError::AllNodesActive);
    //         }
    //
    //         return Ok(collected_nodes);
    //     }
    //
    //     Err(NodePoolError::NodesMapNotInitialized)
    // }

    pub fn set_node_state(
        &mut self,
        node: Gd<Node>,
        old_state: PoolableState,
        new_state: PoolableState,
    ) -> Result<(), NodePoolError> {
        let nodes = self
            .nodes
            .get_mut(&old_state)
            .expect("HashMap states not set.");

        let found_node = nodes
            .iter()
            .position(|node_in_map| node_in_map.instance_id() == node.instance_id())
            .ok_or(NodePoolError::NoNodeFound)?;

        let found_node = nodes.remove(found_node);

        self.nodes
            .get_mut(&new_state)
            .expect("HashMap states not set.")
            .push(found_node);

        Ok(())
    }

    // fn set_node_state(
    //     &mut self,
    //     node_in_map: (NodeState, usize),
    //     new_state: NodeState,
    // ) -> Result<(), NodePoolError> {
    //     let nodes = self
    //         .nodes
    //         .get_mut(&node_in_map.0)
    //         .ok_or(NodePoolError::ErrorInvalidInput)?;
    //
    //     let node = nodes
    //         .get_mut(node_in_map.1)
    //         .ok_or(NodePoolError::ErrorInvalidInput)?;
    //
    //     node.state = new_state;
    //
    //     Ok(())
    // }
}
