
# neatrs

A Rust library for the NEAT algorithm.

Algorithm based on the paper:
<http://nn.cs.utexas.edu/downloads/papers/stanley.ec02.pdf>.


## Documentation

See the doc comments for usage documentation, or download the the repository and
use the command:

`cargo doc --open`

to build and view the rust docs.

**TODO:** Host the docs on the GitHub Pages for this repository.

## Progress

**WARNING:** The library is currently work in progress and NOT ready for any
use.

Core Plan:
 - [x] Create the Environment trait
 - [ ] Create Docs and method signatures for Genome
 - [ ] Implement Organism.rs's key methods
 - [ ] Implement Population.rs's key methods
 - [ ] Implement Genome.rs's outlined methods
 - [ ] Add some basic statistical info to Population

Extra Goals:
 - [ ] Create an example file, where the model learns the XOR function
 - [ ] Create doc-tests, if possible
 - [ ] Allow the user to provide the random library, removing dependancy on a
specific one
