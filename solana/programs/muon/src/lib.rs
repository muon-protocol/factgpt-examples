pub mod types;
pub mod utils;
pub mod errors;

use anchor_lang::prelude::*;
use crate::{
    types::{SchnorrSign, MuonRequestId, GroupPubKey, U256Wrap},
    utils::schnorr_verify,
    errors::MuonError
};

declare_id!("4KBdhmEHx1G5TC4qKqC31DhTLFEe4xrtRhHo6ttwVM7v");

#[program]
pub mod muon {
    use super::*;

    pub fn verify(
    	ctx: Context<Initialize>,
        reqId: MuonRequestId,
        hash: U256Wrap,
        sign: SchnorrSign,
        pubKey: GroupPubKey
    ) -> Result<()> {

        let ret: bool = schnorr_verify(
            // [U256Wrap] signer x
            pubKey.x.val,
            // [u8] signer y parity
            pubKey.parity,
            // [U256Wrap] signature s
            sign.signature.val,
            // [U256Wrap] msg hash
            hash.val,
            // [U256Wrap] nonce address
            sign.nonce.val
        )?;

        if !ret{
            msg!("TSS Not Verified");
            return Err(MuonError::NotVerified.into());
        }

        msg!("req_id: {:x}", reqId);

        //TODO: emit an event.
        // maybe using https://docs.rs/anchor-lang/latest/anchor_lang/macro.emit.html

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // #[account(init, payer = user, space = 8 + 8)]
    // pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// #[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Debug)]
// pub struct MuonRequestId (pub [u8; 32]);

// #[derive(PartialEq)]
// pub struct U256Wrap(pub u256);

// #[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Debug)]
// pub struct SchnorrSign {
//     // s value of signature
//     pub signature: U256Wrap,
//     // ethereum address of nonce
//     pub nonce: U256Wrap
// }

// #[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Debug)]
// pub struct GroupPubKey {
//     // s value of signature
//     pub x: U256Wrap,
//     // ethereum address of signer
//     pub parity: u8
// }

// impl BorshSerialize for U256Wrap {
//     #[inline]
//     fn serialize<W: Write>(&self, writer: &mut W) -> std::result::Result<Self, std::io::Error> {
//         let mut bytes: [u8; 32] = [0; 32];
//         self.0.to_little_endian(&mut bytes);
//         writer.write_all(&bytes)
//     }
// }

// impl BorshDeserialize for U256Wrap {
//     #[inline]
//     fn deserialize(buf: &mut &[u8]) -> std::result::Result<Self, std::io::Error> {
//         // if buf.is_empty() {
//         //     return Err(std::io::Error::new(
//         //         std::io::ErrorKind::InvalidInput,
//         //         "Unexpected length of input",
//         //     ));
//         // }
//         let res = U256Wrap(u256::from_little_endian(&buf[0..32]));
//         *buf = &buf[32..];
//         Ok(res)
//     }
// }
