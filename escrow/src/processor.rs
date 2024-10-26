use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, log::sol_log_compute_units,
    pubkey::Pubkey,
};

use crate::instructions;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct EscrowArgs {
    pub maker: Pubkey,
    pub amount: u64,
    pub receive: u64,
    pub escrow_bump: u8,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum EscrowInstruction {
    Make(EscrowArgs),
    Take,
    Refund,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let instruction = EscrowInstruction::try_from_slice(data)?;

    match instruction {
        EscrowInstruction::Make(escrow_args) => {
            instructions::make(program_id, accounts, escrow_args)?
        }
        EscrowInstruction::Take => instructions::take(program_id, accounts)?,
        EscrowInstruction::Refund => instructions::refund(program_id, accounts)?,
    }

    Ok(())
}
