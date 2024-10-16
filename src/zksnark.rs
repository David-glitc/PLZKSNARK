use bellman::groth16::generate_random_parameters;
use crate::merkle_tree::sequence_transactions;
use crate::state_transition_circuit::StateTransitionCircuit;
use bellman::pairing::bn256::Fr;
use bellman::groth16::{verify_proof,prepare_verifying_key,create_random_proof};



pub fn verify_state_with_proof() -> bool {
    // Step 1: Sequence transactions and generate a Merkle tree
    let transactions = vec![
        Transaction { sender: "Alice".to_string(), receiver: "Bob".to_string(), amount: 50 },
        Transaction { sender: "Bob".to_string(), receiver: "Charlie".to_string(), amount: 20 },
    ];

    let merkle_tree = sequence_transactions(transactions);

    // Step 2: Create zkSNARK proof for the state transition
    let old_state = Fr::from(0);  // Example: initial state
    let new_state = merkle_tree.root();  // New state is the Merkle root

    let circuit = StateTransitionCircuit {
        old_state: Some(old_state),
        new_state: Some(Fr::from(new_state as u64)),  // Representing new state as a field element
        proof: vec![Some(Fr::from(1))],  // Simplified proof
    };

    let proof_valid = generate_proof(circuit);  // Assuming generate_proof is your zkSNARK proof function

    // Step 3: Return the zkSNARK proof result
    proof_valid
}

