#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use processor::process_instruction;
use solana_program::{entrypoint, declare_id};
mod processor {
    use borsh::{BorshDeserialize, BorshSerialize};
    use solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, log::sol_log_compute_units,
        pubkey::Pubkey,
    };
    use crate::instructions;
    pub struct EscrowArgs {
        pub maker: Pubkey,
        pub amount: u64,
        pub receive: u64,
        pub escrow_bump: u8,
    }
    impl borsh::ser::BorshSerialize for EscrowArgs {
        fn serialize<__W: borsh::io::Write>(
            &self,
            writer: &mut __W,
        ) -> ::core::result::Result<(), borsh::io::Error> {
            borsh::BorshSerialize::serialize(&self.maker, writer)?;
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            borsh::BorshSerialize::serialize(&self.receive, writer)?;
            borsh::BorshSerialize::serialize(&self.escrow_bump, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for EscrowArgs {
        fn deserialize_reader<__R: borsh::io::Read>(
            reader: &mut __R,
        ) -> ::core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                maker: borsh::BorshDeserialize::deserialize_reader(reader)?,
                amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                receive: borsh::BorshDeserialize::deserialize_reader(reader)?,
                escrow_bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    pub enum EscrowInstruction {
        Make(EscrowArgs),
        Take(EscrowArgs),
        Refund(EscrowArgs),
    }
    impl borsh::de::BorshDeserialize for EscrowInstruction {
        fn deserialize_reader<__R: borsh::io::Read>(
            reader: &mut __R,
        ) -> ::core::result::Result<Self, borsh::io::Error> {
            let tag = <u8 as borsh::de::BorshDeserialize>::deserialize_reader(reader)?;
            <Self as borsh::de::EnumExt>::deserialize_variant(reader, tag)
        }
    }
    impl borsh::de::EnumExt for EscrowInstruction {
        fn deserialize_variant<__R: borsh::io::Read>(
            reader: &mut __R,
            variant_tag: u8,
        ) -> ::core::result::Result<Self, borsh::io::Error> {
            let mut return_value = if variant_tag == 0u8 {
                EscrowInstruction::Make(
                    borsh::BorshDeserialize::deserialize_reader(reader)?,
                )
            } else if variant_tag == 1u8 {
                EscrowInstruction::Take(
                    borsh::BorshDeserialize::deserialize_reader(reader)?,
                )
            } else if variant_tag == 2u8 {
                EscrowInstruction::Refund(
                    borsh::BorshDeserialize::deserialize_reader(reader)?,
                )
            } else {
                return Err(
                    borsh::io::Error::new(
                        borsh::io::ErrorKind::InvalidData,
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Unexpected variant tag: {0:?}", variant_tag),
                            );
                            res
                        }),
                    ),
                )
            };
            Ok(return_value)
        }
    }
    impl borsh::ser::BorshSerialize for EscrowInstruction {
        fn serialize<__W: borsh::io::Write>(
            &self,
            writer: &mut __W,
        ) -> ::core::result::Result<(), borsh::io::Error> {
            let variant_idx: u8 = match self {
                EscrowInstruction::Make(..) => 0u8,
                EscrowInstruction::Take(..) => 1u8,
                EscrowInstruction::Refund(..) => 2u8,
            };
            writer.write_all(&variant_idx.to_le_bytes())?;
            match self {
                EscrowInstruction::Make(id0) => {
                    borsh::BorshSerialize::serialize(id0, writer)?;
                }
                EscrowInstruction::Take(id0) => {
                    borsh::BorshSerialize::serialize(id0, writer)?;
                }
                EscrowInstruction::Refund(id0) => {
                    borsh::BorshSerialize::serialize(id0, writer)?;
                }
            }
            Ok(())
        }
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
            EscrowInstruction::Take(escrow_args) => {
                ::core::panicking::panic("not yet implemented")
            }
            EscrowInstruction::Refund(escrow_args) => {
                ::core::panicking::panic("not yet implemented")
            }
        }
        Ok(())
    }
}
mod instructions {
    pub mod make {
        use solana_program::{
            account_info::AccountInfo, entrypoint::ProgramResult,
            program_error::ProgramError, pubkey::Pubkey, system_program,
        };
        use crate::{check_id, processor::EscrowArgs};
        pub fn make(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            args: EscrowArgs,
        ) -> ProgramResult {
            let [maker, mint_a, mint_b, escrow, maker_ta_a, vault, token_program,
            system_program] = accounts else {
                return Err(ProgramError::NotEnoughAccountKeys)
            };
            if !system_program::check_id(system_program.key) {
                ::core::panicking::panic(
                    "assertion failed: system_program::check_id(system_program.key)",
                )
            }
            if !spl_token::check_id(system_program.key) {
                ::core::panicking::panic(
                    "assertion failed: spl_token::check_id(system_program.key)",
                )
            }
            if !crate::check_id(program_id) {
                ::core::panicking::panic("assertion failed: crate::check_id(program_id)")
            }
            if !maker.is_signer {
                ::core::panicking::panic("assertion failed: maker.is_signer")
            }
            Ok(())
        }
    }
    pub mod take {}
    pub mod refund {}
    pub use make::*;
    pub use take::*;
    pub use refund::*;
}
/// The const program ID.
pub const ID: ::solana_program::pubkey::Pubkey = ::solana_program::pubkey::Pubkey::new_from_array([
    15u8,
    30u8,
    107u8,
    20u8,
    33u8,
    192u8,
    74u8,
    7u8,
    4u8,
    49u8,
    38u8,
    92u8,
    25u8,
    197u8,
    187u8,
    238u8,
    25u8,
    146u8,
    186u8,
    232u8,
    175u8,
    209u8,
    205u8,
    7u8,
    142u8,
    248u8,
    175u8,
    112u8,
    71u8,
    220u8,
    17u8,
    247u8,
]);
/// Returns `true` if given pubkey is the program ID.
pub fn check_id(id: &::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID.
pub const fn id() -> ::solana_program::pubkey::Pubkey {
    ID
}
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) = unsafe {
        ::solana_program::entrypoint::deserialize(input)
    };
    match process_instruction(&program_id, &accounts, &instruction_data) {
        Ok(()) => ::solana_program::entrypoint::SUCCESS,
        Err(error) => error.into(),
    }
}
