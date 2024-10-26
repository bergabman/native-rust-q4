use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_program,
};
use spl_token::{instruction::{close_account, transfer_checked}, state::Mint};

use crate::state::Escrow;

pub fn take(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let [
        taker, 
        maker, 
        mint_a, 
        mint_b, 
        escrow, 
        maker_ta_b,
        taker_ta_a,
        taker_ta_b,
        vault, 
        token_program, 
        system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(system_program::check_id(system_program.key));
    assert!(spl_token::check_id(system_program.key));
    assert!(crate::check_id(program_id));
    assert!(taker.is_signer);
    assert!(taker.is_writable);

    let mint_a_decimals = Mint::unpack(&mint_a.try_borrow_data()?)?.decimals;
    let mint_b_decimals = Mint::unpack(&mint_b.try_borrow_data()?)?.decimals;

    let escrow_data = *bytemuck::try_from_bytes::<Escrow>(*escrow.data.borrow())
        .map_err(|_| ProgramError::AccountBorrowFailed)?;
    let escrow_seeds = &[b"escrow", maker.key.as_ref(), &[escrow_data.bump as u8]];

    let b_amount = spl_token::state::Account::unpack(&vault.try_borrow_data()?)?.amount;

    // Transfer A from vault to taker_ta_a
    invoke_signed(
        &transfer_checked(
            token_program.key,
            vault.key,
            mint_a.key,
            taker_ta_a.key,
            escrow.key,
            &[],
            b_amount,
            mint_a_decimals,
        )?,
        accounts,
        &[escrow_seeds]
    )?;

    // Transfer B from taker_ta_b
    invoke(
        &transfer_checked(
            token_program.key,
            taker_ta_b.key,
            mint_b.key,
            maker_ta_b.key,
            taker.key,
            &[],
            escrow_data.receive,
            mint_b_decimals,
        )?,
        accounts,
    )?;

    // close escrow
    let mut escrow_data = escrow.data.borrow_mut();
    escrow_data.fill(0);
    let maker_orig_lamports = maker.lamports();
    **maker.lamports.borrow_mut() = maker_orig_lamports.checked_add(escrow.lamports()).ok_or(ProgramError::ArithmeticOverflow)?;
    **escrow.lamports.borrow_mut() = 0;

    // close vault
    invoke_signed(
        &close_account(
            token_program.key,
            vault.key,
            maker.key,
            escrow.key,
            &[]
        )?, 
        accounts,
        &[escrow_seeds],
    )?;

    Ok(())
}
