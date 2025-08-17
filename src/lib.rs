// Minimal trait and constants to satisfy the cards module API
pub trait Arbitrary {
    fn random() -> Self;
}

// Type aliases used by the original module
type Equity = f32;

// Placeholder constants used by `cards::street` API. Values are irrelevant for core cards logic.
pub const KMEANS_FLOP_CLUSTER_COUNT: usize = 0;
pub const KMEANS_TURN_CLUSTER_COUNT: usize = 0;
pub const KMEANS_EQTY_CLUSTER_COUNT: usize = 0;
pub const KMEANS_FLOP_TRAINING_ITERATIONS: usize = 0;
pub const KMEANS_TURN_TRAINING_ITERATIONS: usize = 0;

pub mod cards;

