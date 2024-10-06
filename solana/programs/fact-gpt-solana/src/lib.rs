use anchor_lang::prelude::*;
use muon::{
    types::*,
    self,
};
use muon::program::Muon;
use muon::cpi::accounts::Initialize as MuonInitialize;
use muon::cpi::verify;
use sha3::{Digest, Keccak256};
use primitive_types::U256 as u256;


declare_id!("EciTmTmvfdp8rcJurZ43r7thhnvhZPfA3ChJ8nuYqsC1");

#[program]
pub mod fact_gpt_solana {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,  
        owner: Pubkey, 
        muon_app_info: MuonAppInfo, 
        muon_program: Pubkey
    ) -> Result<()> {
        let state_account = &mut ctx.accounts.state_account;
        let muon_info = &mut ctx.accounts.muon_info;

        state_account.owner = owner;
        muon_info.app_info = muon_app_info;
        muon_info.program_id = muon_program;

        
        Ok(())
    }

    pub fn set_outcome(
        ctx: Context<SetOutcome>, 
        outcome: bool, 
        req_id: MuonRequestId, 
        sign: SchnorrSign,
    ) -> Result<()> {
        let muon_info = &mut ctx.accounts.muon_info;
        let cpi_ctx = CpiContext::new(
            ctx.accounts.muon_program.to_account_info(),
            MuonInitialize {
                user: ctx.accounts.user.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info()
            }
        );
        let mut hasher = Keccak256::new();
    
        let mut bytes: [u8; 32] = [0; 32];
        muon_info.app_info.app_id.val.to_big_endian(&mut bytes);

        hasher.update(&bytes);

        hasher.update(&req_id.val);

        hasher.update(outcome.to_string());
        let result = hasher.finalize();

        let msg_hash = u256::from(&result[..]);
        return verify(cpi_ctx, req_id, U256Wrap { val: msg_hash }, sign, muon_info.app_info.group_pub_key);
    }
}

#[account]
#[derive(Default)]
pub struct StateAccount {
    pub initialized: bool,
    pub owner: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone)]
pub struct MuonAppInfo {
    pub group_pub_key: GroupPubKey,
    pub app_id: U256Wrap
}

#[account]
pub struct MuonInfo {
    pub app_info: MuonAppInfo,
    pub program_id: Pubkey,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = owner,
        space = 1 + 32 + 8, // Extra 8 bytes for the account discriminator that anchor adds 
        seeds = [
            b"state_account"
        ],
        bump
    )]
    pub state_account: Account<'info, StateAccount>,
    #[account(
        init,
        payer = owner,
        space = 72 + 32, seeds = [b"muon_info"], bump
    )]
    pub muon_info: Account<'info, MuonInfo>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct SetOutcome<'info> {
    #[account(
        seeds = [b"muon_info"],
        bump,
    )]
    pub muon_info: Account<'info, MuonInfo>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address = muon_info.program_id)]
    pub muon_program: Program<'info, Muon>,
    pub system_program: Program<'info, System>
}

