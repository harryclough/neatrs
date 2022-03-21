//! Contains structs and functions for working with genomes - a structure that
//! describes a neural network.

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
    pub fn activate(&self, activations: &mut Activations, input: &Vec<f64>)
        -> Vec<f64> {
        return vec![]
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
    /// The `innov` parameter takes the current `Innovation Number`. If a new
    /// gene is added, the innovation number is assigned to that gene.
    /// 
    /// Returns a `usize` that is a count of how many new genes were added. This
    /// will be 0 or 1 for this function. It is likely that you will want to
    /// increment the current `Innovation Number` by this amount.
    /// 
    /// Indices that refer to indices that are in the same layer
    pub fn add_connection(&mut self, in_node: usize, out_node: usize,
        weight: f64, innov: usize) -> usize {
            return 0
    }

    /// Takes the indices of two nodes that have an enabled connection between
    /// them and adds a new node between them. This `disables` the existing
    /// connection and creates two new genes, each describing the connection
    /// into and out of a new node, respectively.
    ///
    /// The `innov` parameter takes the current `Innovation Number`. The first
    /// gene added will be assigned the current value `innov`, and the second
    /// will be assigned the value `innov + 1`.
    /// 
    /// Returns a `usize` that is a count of how many new genes were added. This
    /// will be 0 or 2 for this function.  It is likely that you will want to
    /// increment the current `Innovation Number` by this amount.
    ///
    /// Indices that refer to nodes that do not exist will cause a panic.
    pub fn add_node(&mut self, node_from: usize, node_to: usize,
        innov: usize) -> usize {
            return 0
    }

    /// Calls the [`add_connection()`](Self::add_connection()) method with
    /// random, valid values for `in_node`, `out_node`, and `weight`. The new
    /// weight will be in the range `[0, weight_max]`. Passes on the return
    /// value of `add_connection()`.
    pub fn add_rand_connection(&mut self, weight_max: f64,
        innov: usize) -> usize {
            return 0
    }

    /// Calls the [`add_node()`](Self::add_node()) method with random, valid
    /// values for `node_from` and `node_to`. Passes on the return value of 
    /// `add_node()`.
    pub fn add_rand_node(&mut self, innov: usize) -> usize {
        return 0
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
        p_reassign: f64, reassign_max: f64) {
        return
    }

    /// Returns an [`Activations`] struct, with all inputs and outputs set to
    /// 0.
    pub fn new_activations(&self) -> Activations {
        return Activations{}
    }
}