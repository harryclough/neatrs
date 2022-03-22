//! Contains structs and functions for working with genomes - a structure that
//! describes a neural network.

use crate::innovation::Innovation;

/// A `Genome` is a structure which describes a neural network.
/// 
/// This struct provides implementations not just for describing a `Genome`, but
/// also for evaluating inputs with it, for mating and mutating it, and more.
/// 
/// In this implementation, the network is activated by "stepping" forwards
/// through it. Each call to [`activate()`](Self::activate()) causes all
/// neurones to sum their inputs and produce an output. A network of depth 5
/// will therefore require at least 5 calls to `activate()` in order for the
/// inputs to have propogated fully to the output layer.
pub struct Genome {

}

/// Represents a connection between two nodes
pub struct Connection {
    /// The innovation number of this connection
    pub innov: usize,
    /// The index of the node that the connection comes from
    pub in_node: usize,
    /// The index of the node that the connnection goes into
    pub out_node: usize,
    /// The weight of the connection
    pub weight: f64,
}

/// Stores the current state of the activations of the hidden nodes in a
/// Genome's network.
pub struct Activations {
}

impl Genome {
    /// Passes the values in the `input` vector into the neural network and then
    /// takes one propagation step forwards. All neurones sum their inputs from
    /// the previous call of `activate` and calculate their output. This means
    /// it will often take numerous calls of `activate` before the output that
    /// is relavent to the input will filter through.
    /// 
    /// The [`activations`](Activations) parameter should contain the values of
    /// the previous activation, or be default if no history is to be used.
    /// 
    /// The size of both the input and output `Vec`s is determined by the size
    /// these layers were initiliased to. Providing a `Vec` that is of incorrect
    /// size will result in a panic.
    pub fn activate(&self, activations: &mut Activations, input: &[f64])
        -> Vec<f64> {
        vec![]
        /* Nodes should contain an "input_sum" property and an "output"
         * property. Two passes of all connections are done:
         * 1) For each connection, take the output of the in-node, apply the
         *    connection weight and add it to the input_sum of the out-node.
         * 2) For each connection, take the input_sum and calculate the output.
         *    Then reset the input_sum to 0.
         * Genome should contain two fuctions:
         *  - get_input(node_index)
         *  - get_output(node_index)
         * This allows the input and output nodes to function differently, ie be
         * stored in different places.
         */
    }

    /// Adds a connection between two nodes, whose indices are provided in the
    /// input variables `in_node` and `out_node`. This adds a new gene to the 
    /// genome. Providing nodes that already have a connection between them will
    /// do nothing. Providing nodes that have a `disabled` connection between
    /// them will re-enable that connection.
    /// 
    /// The `innov_n` parameter takes the `Innovation Number` that is to be
    /// assigned to the new gene that is created.
    /// 
    /// Returns an [`Innovation`] that describes what was added.
    /// 
    /// Indices that refer to nodes that do not exist will cause a panic.
    pub fn add_connection(&mut self, in_node: usize, out_node: usize,
        weight: f64, innov_n: usize) -> Innovation
    {
        Innovation::default()
    }

    /// Takes the indices of two nodes that have an enabled connection between
    /// them and adds a new node between them. This `disables` the existing
    /// connection and creates two new genes, describing the connection
    /// into and out of a new node, respectively.
    ///
    /// The `innov_n` parameter takes the `Innovation Number` that is to be
    /// assigned to the new genes that are created. The first gene will be
    /// assgined `innov_n` and the second `innov + 1`.
    /// 
    /// The in-connection will have a weight of 1, and the out-connection will
    /// have a weight that is the same weight as the old connection.
    /// 
    /// Returns an [`Innovation`] that describes what was added.
    ///
    /// Indices that refer to nodes that do not exist or to connections that are
    /// disabled will cause a panic.
    pub fn add_node(&mut self, in_node: usize, out_node: usize, innov_n: usize)
        -> Innovation
    {
        Innovation::default()
    }

    /// Returns a pair of indices of nodes that currently do not have a
    /// connection (disabled or enabled) between them.
    pub fn get_rand_unconnected(&self) -> (usize, usize) {
        (0, 0)
    }

    /// Returns a pair of indices of nodes that currently have an enabled
    /// connection between them.
    pub fn get_rand_connection(&self) -> (usize, usize) {
        (0, 0)
    }

    /// Mutates each weight in the network randomly.
    /// 
    /// `p_uniform` - probability of a given weight being uniformly perturbed.
    /// The amount it is perturbed will be `[-uniform_max, uniform max]`.
    /// 
    /// `p_reassign` - probability of a given weight being reassigned to a new
    /// random number in the range `[0, reassign_max]`.
    /// 
    pub fn mutate_weights(&mut self, p_uniform: f64, uniform_max: f64,
        p_reassign: f64, reassign_max: f64)
    {

    }

    /// Returns an [`Activations`] struct, with all inputs and outputs set to
    /// 0.
    pub fn new_activations(&self) -> Activations {
        Activations{}
    }
}