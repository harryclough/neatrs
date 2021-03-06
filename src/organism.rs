//! Contains the `Organism` struct - an individual that can be evolved to
//! solve a task or problem.

use crate::genome::{Genome, self, Activations};



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
    /// Passes the values in the `input` vector into the neural network and then
    /// takes one propagation step forwards. All neurones sum their inputs from
    /// the previous call of `activate` and calculate their output. This means
    /// it will often take numerous calls of `activate` before the output that
    /// is relavent to the input will filter through.
    /// 
    /// The size of both the input and output `Vec`s is determined by the size
    /// these layers were initialised to. Providing a `Vec` that is of incorrect
    /// size will result in a panic.
    /// 
    /// If you do not want this function to take previous activations into
    /// account, see the [`fresh_activate`](Self::fresh_activate()) function
    /// instead.
    pub fn activate(&mut self, input: &Vec<f64>) -> Vec<f64> {
        self.genome.activate(&mut self.activations, input)
    }
    
    /// Acts just like the [`activate()`](Self::activate()), but it zeroes the
    /// previous activations of the network first. If you want the network to
    /// take into account past activations, use the `activate()` function
    /// instead.
    pub fn fresh_activate(&mut self, input: &Vec<f64>) -> Vec<f64> {
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
    pub fn activate_n(&mut self, input: &Vec<f64>, n: i32) -> Vec<f64> {
        assert!(n > 0, "n must be at 1, but it was {}", n);
        for _ in 0..n {
            let _ = self.genome.activate(&mut self.activations, input);
        }
        self.genome.activate(&mut self.activations, input)
    }

    /// Acts just like the [`activate_n()`](Self::activate_n()) function, but it
    /// zeroes the previous activations of the network first. IF you want the
    /// network to take into account activations that occured before this
    /// function was called, see `activate_n()`.
    pub fn fresh_activate_n(&mut self, input: &Vec<f64>, n: i32) -> Vec<f64> {
        self.activations = self.genome.new_activations();
        self.activate_n(input, n)
    }
}