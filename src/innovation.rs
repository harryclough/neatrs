//! Provides structs and types for storing information about innovations in the
//! genome.

use std::collections::HashMap;

/// Stores the details of an innovation that occured.
#[derive(Hash, PartialEq, Eq)]
pub enum Innovation {
    /// Represents a new connection that was created between two nodes
    Conn(
        /// Index of the connection in-node
        usize,
        /// Index of the connection out-node
        usize
    ),
    /// Represents a node that was added, splitting a connection
    Node(
        /// Index of the prior connection's in-node
        usize,
        /// Index of the prior connection's out-node
        usize
    ),
}

impl Default for Innovation {
    fn default() -> Self {
        Self::Conn(0, 0)
    }
}

/// A [`HashMap`] that maps innovations to their `Innovation Number`. This
/// allows innovations that have already occured in a generation to be given the
/// same innovation number, by checking if they are already in the map.
pub type InnovationMap = HashMap<Innovation, usize>;