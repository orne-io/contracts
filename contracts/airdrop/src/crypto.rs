use cosmwasm_std::{Addr, Uint128};
use sha3::{Digest, Keccak256};
use std::cmp::Ordering;
use std::convert::TryInto;

/// Based on Mars' https://github.com/mars-protocol/mars-periphery/blob/main/contracts/airdrop/src/crypto.rs
pub fn verify_claim(
    account: &Addr,
    amount: Uint128,
    merkle_proof: Vec<String>,
    merkle_root: &str,
) -> bool {
    let leaf = account.to_string() + &amount.to_string();
    let mut hash_str: String;
    let mut hash_buf = Keccak256::digest(leaf.as_bytes())
        .as_slice()
        .try_into()
        .expect("Wrong length");

    for proof in merkle_proof {
        let mut proof_buf: [u8; 32] = [0; 32];
        hex::decode_to_slice(proof, &mut proof_buf).unwrap();
        let proof_buf_str = hex::encode(proof_buf);
        hash_str = hex::encode(hash_buf);

        if proof_buf_str.cmp(&hash_str.clone()) == Ordering::Greater {
            hash_buf = Keccak256::digest(&[hash_buf, proof_buf].concat())
                .as_slice()
                .try_into()
                .expect("Wrong length")
        } else {
            hash_buf = Keccak256::digest(&[proof_buf, hash_buf].concat())
                .as_slice()
                .try_into()
                .expect("Wrong length")
        }
    }

    hash_str = hex::encode(hash_buf);
    merkle_root == hash_str
}
