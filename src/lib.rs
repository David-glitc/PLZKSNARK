//! # Polkadot Light Node with zkSNARK Integration
//!
//! This crate provides a Polkadot light node that can sequence transactions into batches,
//! and then uses zkSNARKs to generate succinct proofs for the validity of the batched transactions.
//! The crate includes functionality for:
//! - Sequencing transactions and generating a Merkle tree root.
//! - Creating zkSNARK proofs of valid state transitions.
//! - Interacting with Polkadot through a light client, ensuring efficient state verification.
//!
//! ## Features
//!
//! - **Light Client**: Fetches block headers from the Polkadot network and sequences transactions.
//! - **zkSNARK Proofs**: Uses zkSNARKs to prove the validity of the sequenced transactions.
//!
//! ## Example Usage
//!
//! ```rust
//! use polkadot_light_zksnark::verify_state_with_proof;
//!
//! fn main() {
//!     let result = verify_state_with_proof();
//!     assert!(result);
//!     println!("State proof verified successfully!");
//! }
//! ```

pub mod light_node;
pub mod zksnark;
pub mod merkle_tree;
pub mod state_transition_circuit;

pub use light_node::create_light_node;
pub use zksnark::verify_state_with_proof;



