//! neatrs provides the ability to evolve neural networks to solve problems or
//! do tasks using the NEAT algorithm.
//! 
//! An implementation of the algorithm as outlined in this paper:
//! <http://nn.cs.utexas.edu/downloads/papers/stanley.ec02.pdf>

#![allow(dead_code)]

pub mod environment;
pub mod organism;
pub mod population;
#[allow(unused_variables)]
pub mod genome;
pub mod neat;