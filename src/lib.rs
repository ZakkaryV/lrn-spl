use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// define the type of state stored in accounts
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct GreetingAccount {
    count: u32,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let greet_message = format!(
        "Hello world! - from a process_instruction in SPL: {}",
        program_id
    );

    msg!(&greet_message);

    let accounts = &mut accounts.iter();
    let account = next_account_info(accounts)?;

    if account.owner != program_id {
        msg!("This program does not own the account");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    greeting_account.count += 1;
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Account greeted {} times", greeting_account.count);

    Ok(())
}

// declare and export the program's entrypoint
solana_program::entrypoint!(process_instruction);

fn main() -> () {}
