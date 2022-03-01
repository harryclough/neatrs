//! Contains the `Organism` struct - an individual that can be evolved to
//! solve a task or problem.



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
/// [fitness](crate::environment::Env::fitness()) function of the
/// [Env](crate::environment::Env) trait.
pub struct Organism {
    
}