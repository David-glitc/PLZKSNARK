use std::hash::Hasher;
use merkle_light::hash::{Algorithm, Hashable};
use merkle_light::merkle::MerkleTree;
use sha2::{Sha256, Digest};
use std::marker::PhantomData;

// Define the hash algorithm
#[derive(Default, Clone)]
pub struct Sha256Algorithm();
#[derive(Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
}
// Function to generate MerkleTree from transactions
pub fn sequence_transactions (txs: Vec<Transaction>) -> MerkleTree<Transaction> {
    MerkleTree::from_data(txs)
}
