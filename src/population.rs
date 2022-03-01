//! Contains structs and functions for creating and working with
//! a pool of [Organisms](crate::organism::Organism).

use crate::organism::Organism;

/// A pool of [Organisms](crate::organism::Organism).
pub struct Population {
    organisms: Vec<Organism>,
    size: usize,
    data: Option<PopData>
}

/* TODO: implement methods and allow returning from Population.
 * PopData is cleared each generation and calculated only on request by the
 * user.
 */
pub struct PopData {
    best: usize,
    worst: usize,
    mean: f64
}