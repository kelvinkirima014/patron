use std::u8;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::Sysvar,
    rent::Rent,
};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CampaignDetails {
    
    pub admin: Pubkey,
    pub name: String,
    pub description: String,
    pub image_link: String,
    pub amount_donated: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct WithdrawRequest {
    pub amount: u64,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // the id of this program on the solana network
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8],  // This is the data we want to process our instruction for
) -> ProgramResult {
    if instruction_data.len() == 0 {
        return Err(ProgramError::InvalidInstructionData);

    } 

    if instruction_data[0] == 0 {
        return create_campaign(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    } else if instruction_data[0] == 1 {
        return withdraw(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
        
    } else if instruction_data[0] == 2 {
        return donate(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    }


    msg!("Didn't find the entrypoint required");
    Err(ProgramError::InvalidInstructionData)
}



fn create_campaign(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    //iterator on an array of accounts
    //related to this entrypoint
    let accounts_iter = &mut accounts.iter();
    
    //account to write data to
    let writing_account = next_account_info(accounts_iter)?;

    //account of the person creating campaign
    let creator_account = next_account_info(accounts_iter)?;

    //makes sure allowed transaction is allowed by creator
    if !creator_account.is_signer{
        msg!("Creator account should be signer");
        return Err(ProgramError::IncorrectProgramId);
    }
   
    //make sure write is owned by solana program account
    if writing_account.owner != program_id{
        msg!("Write account should be owned by solana program account");
        return Err(ProgramError::IncorrectProgramId);
    }
    
    let mut input_data = CampaignDetails::try_from_slice(&instruction_data)
       .expect("Instruction data serialization failed");

    //only creator should be admin
    if input_data.admin != *creator_account.key{
        msg!("Invalid instruction data");
        return Err(ProgramError::InvalidInstructionData);
    }
    
   //minimum balance needded in program account 
   let rent_exemption = Rent::get()?.minimum_balance(writing_account.data_len());
   //make sure we have that much lamports
   if **writing_account.lamports.borrow() < rent_exemption{
        msg!("Balance should exceed rent_exemption");
        return Err(ProgramError::InsufficientFunds);
   }

   //set initial amount to donate as 0
   input_data.amount_donated = 0;

   input_data.serialize(&mut &mut writing_account.data.borrow_mut()[..]);
   
    Ok(())

}

fn withdraw(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let writing_account = next_account_info(accounts_iter)?;
    let admin_account = next_account_info(accounts_iter)?;

    Ok(())

}

fn donate(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    Ok(())

}




    

