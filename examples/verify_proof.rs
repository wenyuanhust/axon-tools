use axon_tools::types::{AxonBlock, Metadata, Proof, Validator, H256};
use serde::de::DeserializeOwned;

fn read_json<T: DeserializeOwned>(path: &str) -> T {
    let json = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&json).unwrap()
}

fn main() {
    let block: AxonBlock = read_json("examples/block.json");
    let proof: Proof = read_json("examples/proof.json");
    let metadata: Metadata = read_json("examples/metadata.json");
    let mut validators = metadata
        .verifier_list
        .iter()
        .map(|v| Validator {
            bls_pub_key:    v.bls_pub_key.clone(),
            address:        v.address,
            propose_weight: v.propose_weight,
            vote_weight:    v.vote_weight,
        })
        .collect::<Vec<_>>();

    let previous_state_root =
        hex::decode("4a2b9bb5cf3fec805e0b376559a639edac7acced05ce5f58ccea49b170c43d10").unwrap();
    let res = axon_tools::verify_proof(
        block,
        H256::from_slice(&previous_state_root),
        &mut validators,
        proof,
    );
    println!("{:?}", res);
}
