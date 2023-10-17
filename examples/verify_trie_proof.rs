use axon_tools::types::H256;
use eth_light_client_in_ckb_prover::Receipts;
use ethers_core::{types::TransactionReceipt, utils::rlp};
// use serde::de::DeserializeOwned;

// fn read_json<T: DeserializeOwned>(path: &str) -> T {
//     let json = std::fs::read_to_string(path).unwrap();
//     serde_json::from_str(&json).unwrap()
// }

fn main() {
    let mut tx_receipts = Vec::<TransactionReceipt>::new();

    {
        let mut receipt = TransactionReceipt::default();
        receipt.transaction_hash = H256::from([0u8; 32]);
        receipt.transaction_index = 0.into();
        // receipt.gas_used = Some(U256::from(100));
        tx_receipts.push(receipt);
    }

    {
        let mut receipt = TransactionReceipt::default();
        receipt.transaction_hash = H256::from([1u8; 32]);
        receipt.transaction_index = 1.into();
        tx_receipts.push(receipt);
    }

    let receipts: Receipts = tx_receipts.into();

    {
        println!("proof of index 0");
        let proof_index = 0 as u64;
        let receipt_proof = receipts.generate_proof(proof_index as usize);

        {
            println!("test key 0");
            let key = rlp::encode(&proof_index);
            let result =
                axon_tools::verify_trie_proof(receipts.root(), &key, receipt_proof.clone());
            println!("key: {:?}, result: {:?}", key, result);
            assert!(result.unwrap().is_some());
        }

        {
            println!("test key 1");
            let key = rlp::encode(&(1 as u64));
            let result =
                axon_tools::verify_trie_proof(receipts.root(), &key, receipt_proof.clone());
            println!("key: {:?}, result: {:?}", key, result);
            assert!(result.unwrap().is_none());
        }

        {
            println!("test key 2");
            let key = rlp::encode(&(2 as u64));
            let result =
                axon_tools::verify_trie_proof(receipts.root(), &key, receipt_proof.clone());
            println!("key: {:?}, result: {:?}", key, result);
            assert!(result.unwrap().is_none());
        }

        {
            println!("test illegal trie root");
            let key = rlp::encode(&(200 as u64));
            let result =
                axon_tools::verify_trie_proof(H256::from([4u8; 32]), &key, receipt_proof.clone());
            println!("key: {:?}, result: {:?}", key, result);
            assert!(result.is_err());
        }
    }

    {
        println!("proof of index 1, wrong");
        let proof_index = 1 as u64;
        let receipt_proof = receipts.generate_proof(proof_index as usize);

        {
            println!("test key 0");
            let key = rlp::encode(&(0 as u64));
            let result =
                axon_tools::verify_trie_proof(receipts.root(), &key, receipt_proof.clone());
            println!("key: {:?}, result: {:?}", key, result);
            assert!(result.unwrap().is_none());
        }

        {
            println!("test key 1");
            let key = rlp::encode(&(1 as u64));
            let result =
                axon_tools::verify_trie_proof(receipts.root(), &key, receipt_proof.clone());
            println!("key: {:?}, result: {:?}", key, result);
            assert!(result.unwrap().is_some());
        }
    }
}
