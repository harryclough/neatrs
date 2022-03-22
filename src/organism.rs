//! Contains the `Organism` struct - an individual that can be evolved to
//! solve a task or problem.

use std::collections::hash_map::Entry::*;

use crate::genome::{Genome, Activations};
use crate::innovation::{InnovationMap, Innovation};

use rand::{thread_rng, Rng};


/// An `Organism` is a individual that through an internal neural network
/// attempts to perform a task or solve a problem.
/// 
/// The neural network is defined through genes within the `Organism`, which are
/// passed on and modified when the organism produces offspring.
/// 
/// Most of the behaviour of the `Organism` is used in the training process,
/// where it is bred, mutated and evaluated.
/// 
/// The fitness of an `Organism` should be assessed through the
/// [`fitness`](crate::environment::Environment::fitness()) function of the
/// [`Environment`](crate::environment::Environment) trait.
pub struct Organism {
    genome: Genome,
    activations: Activations,
}

impl Organism {
    /// Calls the [`activate()`](Genome::activate()) function of this organism's
    /// [`Genome`], taking into account previous activations of the network.
    /// 
    /// If you do not want this function to take previous activations into
    /// account, see the [`fresh_activate`](Self::fresh_activate()) function
    /// instead.
    pub fn activate(&mut self, input: &[f64]) -> Vec<f64> {
        self.genome.activate(&mut self.activations, input)
    }
    
    /// Acts just like the [`activate()`](Self::activate()), but does not take
    /// into account the prior activations of the newtwork. If you want the
    /// network to take into account past activations, use the `activate()`
    /// function instead.
    pub fn fresh_activate(&mut self, input: &[f64]) -> Vec<f64> {
        self.activations = self.genome.new_activations();
        self.activate(input)
    }

    /// Calls the [`activate()`](Self::activate()) function n times with the
    /// given `input` layer. (where n >= 1) This can be useful if you want the
    /// network to fully propagate forwards, instead of just one step.
    /// 
    /// For example, if the network has a maximum depth of 10, it will take at
    /// least 10 calls of the `activate()` function before the new inputs have
    /// fully propagated to the output layer.
    /// 
    /// If the network contains loops of nodes, it may be worth calling the
    /// activate function many times, to allow the output to stabalise.
    pub fn activate_n(&mut self, input: &[f64], n: i32) -> Vec<f64> {
        assert!(n > 0, "n must be at 1, but it was {}", n);
        for _ in 0..n {
            let _ = self.genome.activate(&mut self.activations, input);
        }
        self.genome.activate(&mut self.activations, input)
    }

    /// Acts just like the [`activate_n()`](Self::activate_n()) function, but
    /// clears the previous activations of the network before it starts.
    /// 
    /// TODO: CHANGE TO BE PARAMETER TO ACTIVATE
    pub fn fresh_activate_n(&mut self, input: &[f64], n: i32) -> Vec<f64> {
        self.activations = self.genome.new_activations();
        self.activate_n(input, n)
    }

    /// Randonly mutates this individual's Genome, based on the parameters given
    /// in `params`.
    /// 
    /// Mutations include:
    /// - Adding a new connection between two nodes.
    /// - Adding a new node, splitting a connection.
    /// - Purturbing / reassigning network weights.
    /// 
    /// Will modify the `Innovation Number` `innov_n` and `innov_map` as
    /// innovations are made.
    pub fn mutate(&mut self, innov_n: &mut usize, innov_map: &mut InnovationMap,
        params: &MutationParams)
    {
        // New connection
        let random: f64 = thread_rng().gen();
        if random < params.p_add_connection {
            self.new_rand_connection(innov_n, innov_map, params);
        }
        // New node
        let random: f64 = thread_rng().gen();
        if random < params.p_add_node {
            self.new_rand_node(innov_n, innov_map);
        }
        // Mutate weights
        let random: f64 = thread_rng().gen();
        if random < params.p_mut_weights {
            self.genome.mutate_weights(
                params.p_uniform, params.uniform_max,
                params.p_reassign, params.rand_weight_max
            );
        }
    }

    /// Generates a new, random connection between two previously unconnected
    /// nodes. This creates 1 gene/innovation.
    /// 
    /// If the innovation is not already in `innov_map`, it will be added and 
    /// the `Innovation Number` `innov_n` will be incremented by 1. Otherwise,
    /// the same `Innovation Number` as is in the map will be used.
    fn new_rand_connection(&mut self, innov_n: &mut usize,
        innov_map: &mut InnovationMap, params: &MutationParams)
    {
        let (in_node, out_node) = self.genome.get_rand_connection();
        let weight = thread_rng().gen::<f64>() * params.rand_weight_max;
        let innovation = Innovation::Conn(in_node, out_node);
        match innov_map.entry(innovation) {
            Occupied(entry) => {
                let innov_old = *entry.get();
                self.genome.add_connection(in_node, out_node, weight, innov_old);
            }
            Vacant(entry) => {
                self.genome.add_connection(in_node, out_node, weight, *innov_n);
                entry.insert(*innov_n);
                *innov_n += 1;
            }
        }
    }

    /// Generates a new, random node between two previously connected
    /// nodes. This creates 2 genes/innovations.
    /// 
    /// If the innovations are not already in `innov_map`, it will be added and 
    /// the `Innovation Number` `innov_n` will be incremented by 2. Otherwise,
    /// the same `Innovation Number`s as are in the map will be used.
    fn new_rand_node(&mut self, innov_n: &mut usize,
        innov_map: &mut InnovationMap)
    {
        let (in_node, out_node) = self.genome.get_rand_connection();
        let innovation = Innovation::Node(in_node, out_node);
        match innov_map.entry(innovation) {
            Occupied(entry) => {
                let innov_old = *entry.get();
                self.genome.add_node(in_node, out_node, innov_old);
            }
            Vacant(entry) => {
                self.genome.add_node(in_node, out_node, *innov_n);
                entry.insert(*innov_n);
                *innov_n += 1;
            }
        }
    }
}

/// Contains the parameters that define when and how to mutate a genome.
pub struct MutationParams {
    /// Probability of adding a new connection.
    pub p_add_connection: f64,
    /// Probability of adding a new node.
    pub p_add_node: f64,
    ///Probability of mutating the genome's weights.
    pub p_mut_weights: f64,
    /// Probability a weight is perturbed uniformly in the range
    /// `[-uniform_max, uniform_max]`
    pub p_uniform: f64,
    pub uniform_max: f64,
    /// Probability a weight is set to a new random value.
    pub p_reassign: f64,
    /// When nodes are assigned a new random value, they are assigned in the
    /// range `[0, rand_weight_max]`.
    pub rand_weight_max: f64,
}