use primitive_types::{U256 as u256, U512 as u512};
use hex_literal::hex;
use sha3::{Digest, Keccak256};

use solana_program::{
    secp256k1_recover::{
        secp256k1_recover
    }
};

use crate::{
    errors::MuonError
};

const Q_BYTES:[u8; 32] = hex!("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141");
fn mod_neg(a: u256, q: u256) -> u256 {
    let mut res = a;
    if a > q {
        res = a % q;
    }
    (q - res) % q
}

#[allow(dead_code)]
fn mod_add(a: u256, b: u256, q: u256) -> u256 {
    let res = (u512::from(a) + u512::from(b)) % u512::from(q);
    let mut b_arr: [u8; 64] = [0; 64];
    res.to_big_endian(&mut b_arr);
    u256::from(&(b_arr[32..64]))
}

fn mod_mul(a: u256, b: u256, q: u256) -> u256 {
    let res = (u512::from(a) * u512::from(b)) % u512::from(q);
    let mut b_arr: [u8; 64] = [0; 64];
    res.to_big_endian(&mut b_arr);
    u256::from(&(b_arr[32..64]))
}

pub fn schnorr_verify(
    signing_pubkey_x: u256,
    signing_pubkey_y_parity: u8,
    signature_s: u256,
    msg_hash: u256,
    nonce_address: u256
) -> Result<bool, MuonError> {

    let q:u256 = u256::from_big_endian(&Q_BYTES);
    let q_half = (q >> 1) + 1;

    if signing_pubkey_x >= q_half {
        return Err(MuonError::LargePubkeyX)
    }

    if signing_pubkey_x.is_zero() || nonce_address.is_zero() || msg_hash.is_zero() || signature_s.is_zero() {
        return Err(MuonError::ZeroSignatureData)
    }

    let e = make_msg_challenge(nonce_address, msg_hash).unwrap();

    let args_z: u256 = mod_mul(mod_neg(signing_pubkey_x, q), signature_s, q);

    let args_v: u8 = signing_pubkey_y_parity;
    let args_r: u256 = signing_pubkey_x;
    let args_s: u256 = mod_mul(signing_pubkey_x, e, q);

    let args_rs: u512 = (u512::from(args_r) << 256) + args_s;

    let mut zb: [u8; 32] = [0; 32];
    args_z.to_big_endian(&mut zb);

    let mut rs: [u8; 64] = [0; 64];
    args_rs.to_big_endian(&mut rs);

    let result = secp256k1_recover(&zb, args_v, &rs);
    let nonce_address = pub_to_eth_address(&(result.unwrap().0));

    let e_2 = make_msg_challenge(nonce_address, msg_hash).unwrap();

    Ok(e_2 == e)
}

fn pub_to_eth_address(pubkey: &[u8; 64]) -> u256 {
    let mut hasher = Keccak256::new();
    hasher.update(pubkey);
    let result = hasher.finalize();
    let mut _hash:u256 = u256::from(&result[..]);
    _hash = _hash & u256::from("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
    return _hash;
}


fn make_msg_challenge (
    nonce_times_generator_address: u256, msg_hash: u256
) -> Result<u256, MuonError> {
    let mut hasher = Keccak256::new();
    let nonce_bytes:[u8; 32] = nonce_times_generator_address.into();
    let hash_bytes:[u8; 32] = msg_hash.into();
    // last 20 bytes will be used to hash
    hasher.update(&nonce_bytes[12..32]);
    hasher.update(&hash_bytes);
    let result = hasher.finalize();
    let _hash:u256 = u256::from(&result[0..32]);
    return Ok(_hash);
}
