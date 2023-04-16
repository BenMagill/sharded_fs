use std::{collections::HashSet, hash::Hash};

/**
 * Try to connect to a node that's address is provided
 * Get list of nodes from this node
 * 
 * On node connecting to another, tell all other nodes
 * 
 */

struct NodeManager {
    nodes: HashSet<String>,
}

impl NodeManager {
    // Only use when no address provided (starting from scract)
    fn new() -> NodeManager {
        NodeManager {
            nodes: HashSet::new()
        }
    }

    fn connect(addr: &str) -> NodeManager {
        /**
         * Call address provided
         * Get list of addresses from it
         * Store these
         */
    }

    fn broadcast_new_node(addr: &str) {
        /**
         * On a node connecting to thise one, tell all others
         */
    }

    
}