//! Provides controls for building and running the NEAT algorithm.

/// The `NEAT` struct provides an easy way to setup and train a neat algorithm,
/// handling most of the steps in the process. The use of this struct is
/// optional, and advanced user may prefer not to use it at all.
struct NEAT {

}

/// This trait allows any random number library to be used.
trait Rand {
    /// This function should return a (pseudo) random number in the range 
    /// `[0, 1)`. That is to say between `0` and `1`, inclusive of `0` but
    /// exclusive of `1`.
    /// 
    /// e.g., the `rand` library's `rand::Rng::gen` method.
    fn generate() -> f64;
}