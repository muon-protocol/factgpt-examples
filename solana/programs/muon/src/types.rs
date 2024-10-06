// use borsh::{BorshDeserialize, BorshSerialize};
use self::borsh::maybestd::io::Write;
use primitive_types::{U256 as u256};
use std::{
    convert::TryInto,
    fmt
};

use anchor_lang::prelude::*;


#[derive(PartialEq, Copy, Clone)]
pub struct U256Wrap{
    pub val: u256
}


//#[derive(BorshSerialize, BorshDeserialize, Debug)]
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug)]
pub struct SchnorrSign {
    // s value of signature
    pub signature: U256Wrap,
    // ethereum address of nonce
    pub nonce: U256Wrap
}

//#[derive(BorshSerialize, BorshDeserialize, Debug)]
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Copy, Clone)]
pub struct GroupPubKey {
    // s value of signature
    pub x: U256Wrap,
    // ethereum address of signer
    pub parity: u8
}

//#[derive(BorshSerialize, BorshDeserialize, Debug)]
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug)]
pub struct MuonAppInfo {
    pub group_pub_key: GroupPubKey,
    pub muon_app_id: U256Wrap
}

//#[derive(BorshSerialize, BorshDeserialize, Debug)]
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug)]
pub struct SchnorrVerifyInstruction {
    pub signing_pubkey_x: U256Wrap,
    pub signing_pubkey_y_parity: u8,
    pub signature_s: U256Wrap,
    pub msg_hash: U256Wrap,
    pub nonce_address: U256Wrap
}

// #[derive(BorshSerialize, BorshDeserialize, Debug)]
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug)]
pub struct MuonRequestId{
    pub val: [u8; 32]
}

impl fmt::LowerHex for MuonRequestId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "f")?;
        for i in &self.val[..] {
            write!(f, "{:02x}", i)?;
        }
        Ok(())
    }
}

// impl BorshSerialize for MuonRequestId {
//     #[inline]
//     fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
//         writer.write_all(&self.0)
//     }
// }

// impl BorshDeserialize for MuonRequestId {
//     #[inline]
//     fn deserialize(buf: &mut &[u8]) -> Result<Self, std::io::Error> {
//         if buf.is_empty() {
//             return Err(std::io::Error::new(
//                 std::io::ErrorKind::InvalidInput,
//                 "Unexpected length of input",
//             ));
//         }
//         let res: [u8; 32] = buf[0..32].try_into().expect("slice with incorrect length");
//         *buf = &buf[32..];
//         Ok(MuonRequestId(res))
//     }
// }

// impl std::fmt::Debug for MuonRequestId {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{:02x?}", self.0)
//     }
// }

impl AnchorSerialize for U256Wrap {

    fn serialize<W: Write>(&self, writer: &mut W) -> std::result::Result<(), std::io::Error> {
        let mut bytes: [u8; 32] = [0; 32];
        self.val.to_little_endian(&mut bytes);
        writer.write_all(&bytes)
    }
}

impl AnchorDeserialize for U256Wrap {
    fn deserialize(buf: &mut &[u8]) -> std::result::Result<Self, std::io::Error> {
        // if buf.is_empty() {
        //     return Err(std::io::Error::new(
        //         std::io::ErrorKind::InvalidInput,
        //         "Unexpected length of input",
        //     ));
        // }
        let res = U256Wrap{val: u256::from_little_endian(&buf[0..32])};
        *buf = &buf[32..];
        Ok(res)
    }
    
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        todo!()
    }
}

impl std::fmt::LowerHex for U256Wrap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.val)
    }
}

impl std::fmt::Debug for U256Wrap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.val)
    }
}
