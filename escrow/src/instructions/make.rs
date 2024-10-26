use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction, system_program,
    sysvar::Sysvar,
};
use spl_token::{instruction::transfer_checked, state::Mint};

use crate::{processor::EscrowArgs, state::Escrow};

pub fn make(program_id: &Pubkey, accounts: &[AccountInfo], args: EscrowArgs) -> ProgramResult {
    let [maker, mint_a, mint_b, escrow, maker_ta_a, vault, token_program, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(system_program::check_id(system_program.key));
    assert!(crate::check_id(program_id));
    assert!(maker.is_signer);
    assert!(maker.is_writable);
    // assert_eq!(mint_a.owner, token_program.key);
    // assert_eq!(mint_b.owner, token_program.key);
    // assert_eq!(maker_ta_a.owner, token_program.key);
    assert_eq!(vault.owner, system_program.key);

    let mint_unpacked = Mint::unpack(&mint_a.try_borrow_data()?)?;

    assert!(escrow.is_writable && escrow.data_is_empty());
    let escrow_seeds = &[b"escrow", maker.key.as_ref(), &[args.escrow_bump]];
    let expected_escrow = Pubkey::create_program_address(escrow_seeds, program_id)?;
    assert_eq!(&expected_escrow, escrow.key);

    invoke_signed(
        &system_instruction::create_account(
            maker.key,
            escrow.key,
            Rent::get()?.minimum_balance(Escrow::LEN),
            Escrow::LEN as u64,
            &crate::id(),
        ),
        accounts,
        &[escrow_seeds],
    )?;

    let new_escrow = Escrow {
        maker: *maker.key,
        mint_a: *mint_a.key,
        mint_b: *mint_b.key,
        receive: args.receive,
        bump: args.escrow_bump as u64,
    };

    let mut escrow_data = *bytemuck::try_from_bytes_mut::<Escrow>(*escrow.data.borrow_mut())
        .map_err(|_| ProgramError::AccountBorrowFailed)?;

    escrow_data.clone_from(&new_escrow);

    // Transfer to vault
    invoke(
        &transfer_checked(
            token_program.key,
            maker_ta_a.key,
            mint_a.key,
            vault.key,
            maker.key,
            &[],
            args.amount,
            mint_unpacked.decimals,
        )?,
        &[
            maker.clone(), 
            maker_ta_a.clone(),
            vault.clone()
            ],
    )?;

    Ok(())
}
