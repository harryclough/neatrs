//! Contains the `Env` trait - a trait that allows for the assement of the
//! fitness of a particular `Organism` in an environment.

use crate::organism::Organism;

/// An Env is the environment in which an [Organism](Organism) is learning to
/// perform in, and where it's fitness function is evaluated.
/// 
/// This trait allows for the evalation of the fitness of an `Organism`.
pub trait Environment {
    /// Evaluates and returns the fitness score of the `Organism` provided.
    /// 
    /// This function is used by the NEAT algorithm to compare organisms against
    /// each other. The only assumption that is made about the fitness score
    /// is that a higher fitness score is better.
    fn fitness(&self, organism: Organism) -> f64;
}