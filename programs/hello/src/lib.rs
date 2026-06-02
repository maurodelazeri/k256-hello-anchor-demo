//! Hello world — Anchor program.
//!
//! A k256 Program OS starter: one instruction, a generated IDL, the fullest
//! path through build → IDL → deploy. `declare_id!` is replaced with your real
//! program id when you create the program on chain.

use anchor_lang::prelude::*;

declare_id!("AZutZ4iRx3fX86bU4gGEkxLbQGFPxj6ZQuEDokvTxVjM");

#[program]
pub mod hello {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("hello from k256 Program OS — Anchor program");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
