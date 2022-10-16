use std::vec;

use anchor_lang::prelude::*;
use openssh_keys::PublicKey;

declare_id!("4tcEoG3qEP4LHkdboUCJvusV4hJidiFAtz1knfngwMFd");

#[program]
pub mod team_dao_voting_program {

    use anchor_lang::{solana_program::entrypoint::ProgramResult};

    use super::*;

    pub fn create_team(
        ctx:Context<CreateTeam>,
        team_size:u8,
        team_players_selected:Vec<String>,
        team_captain_selected:String,
        // creating team with team size, team players and team captain selected
    ) -> Result<()>{
        let team_account = &mut ctx.accounts.owner_account;
        // team size must be greater than 2
        if team_size <= 2{
           return Err(error!(Errors::TeamSizeError))
        }
        // team size and selected players must be equal
        if team_size != team_players_selected.len() as u8{
          return  Err(error!(Errors::TeamSizePlayersError))
        }


        team_account.team_size = team_size as u8;
        let mut team_size_count = team_size as usize;
        //with specificied team size we connect our vector to account data
        for i in  0..team_size_count{
            let mut string_variable = String::new();
            string_variable = team_players_selected[i].clone();
            team_account.team_players.push(string_variable);
        }

        team_account.team_captain = team_captain_selected;
        Ok(())
    }

    pub fn create_bump(ctx: Context<CreateTeam>, bump: u8) -> ProgramResult {
        ctx.accounts.owner_account.bump = bump;
        Ok(())
      }

    pub fn create_captain(ctx: Context<CreateCaptain>,team_captain_selected:String,captain_pubkey:Pubkey) -> Result<()> {
        let team_captain = &mut ctx.accounts.owner_account;
        //team sizem must be equal or greater than 2
        if team_captain.team_players.len() >= 2 && !team_captain.team_players.contains(&team_captain_selected){
           return Err(error!(Errors::TeamCreationError));
        }

        // selecting captain of the team
        team_captain.team_captain = team_captain_selected;
        // getting pubkey of the team captain
        team_captain.team_captain_account = captain_pubkey;
        msg!("hello {}",team_captain.team_captain);

        Ok(())
    }


    //creating password for captain for further process

    pub fn create_captain_password(ctx:Context<CreateCaptain>,selected_password:String) -> Result<()>{
        let team_account = &mut ctx.accounts.owner_account;
        let length_password = selected_password.chars().count() as usize;
        // symbols for special characters
        // used on transaction to check password from captain account
        let symbols = vec!['`','~','!','@','#','$','%','^','&','*','(',')','_','-','+','=','{','[','}','}','|','\'',':',';','\\','<',',','>','.','?','/'];
        if (length_password <= 8){
            return Err(error!(Errors::CaptainPasswordError))
        }

        // checking all the char types and equal to 8
        let mut big_letters = 0;
        let mut lower_letters = 0;
        let mut special_letters = 0;
        let mut number_type = 0;
        for i in 0..length_password {
            if selected_password.chars().nth(i).unwrap().is_uppercase(){
                big_letters += 1
            }
            if selected_password.chars().nth(i).unwrap().is_lowercase(){
                lower_letters += 1
            }
            if symbols.contains(&selected_password.chars().nth(i).unwrap()){
                special_letters += 1
            }
            if selected_password.chars().nth(i).unwrap().is_digit(10){
                number_type += 1
            }
        }

        // for every type of varialbe check if equal to 3 or more

        if big_letters <= 2{
            return Err(error!(Errors::UpperCaseLettersError))
        }
        if lower_letters <= 2{
            return Err(error!(Errors::LowerCaseLettersError))
        }
        if special_letters <= 2{
            return Err(error!(Errors::SpecialCharactersError))
        }
        if number_type <= 2{
            return Err(error!(Errors::NumLettersError))
        }

        // if all types matches the conditions set the password

        team_account.team_captain_password = selected_password;

        Ok(())
    }

    // If team members would like to vote and get accept team captain selection and similar
    pub fn team_members_accept_vote(ctx:Context<GeneralData>,accept_votes:Vec<bool>) -> Result<()>{
        let team_account = &mut ctx.accounts.team_account;

        if accept_votes.len() != team_account.team_players.len(){
            return Err(error!(Errors::TeamSizeError))
        }

        team_account.accept_votes = accept_votes;
    }


    pub fn get_votes(ctx:Context<GeneralData>,votes:Vec<bool>) -> Result<()>{
        let team_account = &mut ctx.accounts.owner_account;
        let tournement_account & mut ctx.accounts.tournement_account;

        // set prize distributed as false whilst voting
        tournement_account.prize_distrubed = false;

        // checking if given votes and team account players are same length of vector
        if team_account.team_players.len() != votes.len(){
            return Err(error!(Errors::VoteNumbersTeamSizeError));
        }
        //check if all team members want to vote
        let mut count = 0;
        for i in 0..team_account.accept_votes{
            count += team_account.accept_votes[i] as u8;
        }

        if count as usize != team_account.accept_votes{
            return Err(error!(Errors::AcceptVotingError))
        }
        team_account.team_size;
        team_account.team_votes = votes;
        Ok(())
    }

    pub fn votes_calculate(ctx:Context<GeneralData>) -> Result<()> {
        let team_account = &mut ctx.accounts.owner_account;
        let tournement_account = &mut ctx.accounts.tournement_account;
        // negative false sums and true values are calculated
        let mut negative_votes = 0.0;
        let mut positive_votes = 0.0;
        let count = team_account.team_votes.len() as usize;
        if count > 2 {
            msg!("accounts have greater than 2 votes");
            for i in 0..count{
                if team_account.team_votes[i] == true{
                    // adding dist to positive vote segment with distribution
                    positive_votes +=  tournement_account.prize_dist[i] as f64;
                }else{
                    // adding dist to negative vote segment with distribution
                    negative_votes +=  tournement_account.prize_dist[i] as f64;
                }
            }

            // the result dist bigger than 40%;
            // majority 40% calculation with prize dist
            if(positive_votes > 0.4) {
                tournement_account.voted_result = true;
            }else{
                tournement_account.voted_result = false;
            }
        }else{
            return Err(error!(Errors::TeamSizeError));
        }

        Ok(())
    }

    pub fn join_to_team(ctx:Context<GeneralData>,add_people: Vec<String>) -> Result<()>{
        let team_account = &mut ctx.accounts.owner_account;
        // team size must be smaller than 20 people
        if team_account.team_players.len() + add_people.len() >= 20{
            return Err(error!(Errors::TeamSizeSmallerThan20Error));
        }
        for i in 0..add_people.len(){
            // adding team persona to team_players vector list
            team_account.team_players.push(add_people[i].clone());
        }
        Ok(())
    }

    pub fn leave_from_team(ctx:Context<GeneralData>,add_people: Vec<String>) -> Result<()>{
        let team_account = &mut ctx.accounts.owner_account;

        // checking if team has zero people
        if team_account.team_players.len() == 0{
            return Err(error!(Errors::TeamLengthZeroError))
        }

        for i in 0..add_people.len(){
            //checking people if they are in the team
            if !team_account.team_players.contains(&add_people[i]){
                return Err(error!(Errors::ThereisNoSuchTeamMemberError));
            }
            let index = team_account.team_players.iter().position(|r| r.to_string() == add_people[i].to_string()).unwrap();
            team_account.team_players.remove(index);
        }
        Ok(())
    }

    // calculating prize according to players length
    pub fn prize_calculate(ctx:Context<GeneralData>,prize_dist:Vec<u8>) -> Result<()>{
        let team_account = &mut ctx.accounts.owner_account;
        let tournement_account =  &mut ctx.accounts.tournement_account;
        

        // checking if prize dist list equal to team players length
        if prize_dist.len() != team_account.team_players.len() {
            return Err(error!(Errors::PrizeLengthMatchTeamPlayersLengthError))
        }
        if prize_dist.len() >= 2{
            let sum: u8 = prize_dist.iter().sum();
            if sum != 100 {
                return Err(error!(Errors::PrizeDistTotalMustEqual100Error))
            }else{
                tournement_account.prize_dist = prize_dist;
            }
        }else{
            return Err(error!(Errors::PrizeDistLengthGreaterTHan2Error))
        }
        Ok(())
    }

    // Getting publicKeys of Team Players
    pub fn get_pubkeys(ctx:Context<GeneralData>,get_team_pubkeys:Vec<Pubkey>) -> Result<()>{
        let team_account = &mut ctx.accounts.owner_account;
        let tournement_account = &mut ctx.accounts.tournement_account;
        if get_team_pubkeys.len() <= 2 {
            return Err(error!(Errors::TeamSizeError))
        }
        //check if publickey is valid
        for i in 0..get_team_pubkeys.len(){
            let key = openssh_keys::PublicKey::parse(get_team_pubkeys[i]).unwrap();
            let out = key.to_string()
            if out.len() >= 32 && out.len() <= 44{
                return Err(error!(Errors::PubkeyLengthError))
            }
        }
        if get_team_pubkeys.len() != team_account.team_players.len() {
            return Err(error!(Errors::PubkeysLengthMatchTeamPlayersLengthError))
        }
        for i in 0..tournement_account.prize_dist.len(){
            team_account.team_pub_keys.push(get_team_pubkeys[i].clone())
        }

        Ok(())
    }

    //transfer sol quantity to captain
    pub fn transfer_sol_to_captain(ctx:Context<GeneralData>,lamport_for_transfer:u64,selected_captain_password:String) -> Result<()> {
        let team_account = &mut ctx.accounts.owner_account;
        let tournement_account = &mut ctx.accounts.tournement_account;
        // from tournement selected account, transfer sol to captain account
        if selected_captain_password != team_account.team_captain_password{
            // checking if password matches captain password
            return Err(error!(Errors::TeamCreationError));
        }
        let amount = lamport_for_transfer * 1000000;
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &tournement_account.transfer_account.key(),
            &team_account.team_captain_account.key(),
            amount,
        );
        // different accounts could be error or not
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                team_account.to_account_info(),
                tournement_account.to_account_info(),
            ]
        ).err();

        Ok(())
    }

    //From captain to other pubkeys the prize
    pub fn transfer_native_sol(ctx: Context<GeneralData>,selected_captain_password:String) -> Result<()> {
        let team_account = &mut ctx.accounts.owner_account;
        let tournement_account = &mut ctx.accounts.tournement_account;
        if selected_captain_password != team_account.team_captain_password{
             // checking if password matches captain password
            return Err(error!(Errors::TeamCreationError));
        }
        let lamport_num = 10000000;
        for i in 0..team_account.team_pub_keys.len(){
            let ix = anchor_lang::solana_program::system_instruction::transfer(
                &team_account.team_captain_account.key(),
                &team_account.team_pub_keys[i].key(),
                team_account.team_prize as u64 * tournement_account.prize_dist[i] as u64 / 100 * lamport_num,
            );
        // signing team account and tournement account as responsible for transaction
        // different accounts could be error or not
            anchor_lang::solana_program::program::invoke(
                &ix,
                &[
                    team_account.to_account_info(),
                    tournement_account.to_account_info(),
                ],
            ).err()

            // if prize distributed from team captain this variable must be true
            tournement_account.prize_distrubed = true



        }
        Ok(())
    }






    //resources
    // https://stackoverflow.com/questions/70528742/how-to-transfer-sol-in-anchor-smart-contract-instruction
    // https://stackoverflow.com/questions/71086845/solana-token-transfer-using-anchor
    // https://docs.rs/anchor-lang/latest/anchor_lang/derive.Accounts.html#instruction-attribute
    // https://stackoverflow.com/questions/72379951/how-do-i-force-solana-anchor-methods-to-use-the-devnet
    // https://solana.stackexchange.com/questions/3429/sol-transfer-via-cpi-signed-with-pda-throws-fails/3434#3434






}


// Error Messages
#[error_code]
pub enum Errors {
    #[msg("Team captain must be a team member also")]
    TeamCreationError,
    #[msg("Team size must be greater than 2")]
    TeamSizeError,
    #[msg("team size and team players selected should be equal")]
    TeamSizePlayersError,
    #[msg("vote number and team player number must be equal")]
    VoteNumbersTeamSizeError,
    #[msg("team size must be smaller than 20")]
    TeamSizeSmallerThan20Error,
    #[msg("Team length is zero")]
    TeamLengthZeroError,
    #[msg("There is no such a team member")]
    ThereisNoSuchTeamMemberError,
    #[msg("prize dist len must match team players length")]
    PrizeLengthMatchTeamPlayersLengthError,
    #[msg("prize dist total should equal to 100")]
    PrizeDistTotalMustEqual100Error,
    #[msg("prize dist len should be greater than 2")]
    PrizeDistLengthGreaterTHan2Error,
    #[msg("pubkey values match team players length")]
    PubkeysLengthMatchTeamPlayersLengthError,
    #[msg("captain password must longer than 7 characters")]
    CaptainPasswordError,
    #[msg("password should contain 2 of uppercase letters")]
    UpperCaseLettersError,
    #[msg("password should contain 2 of number letters")]
    NumLettersError,
    #[msg("password should contain 2 of special letters")]
    SpecialCharactersError,
    #[msg("password should contain 2 of lowercase letter letters")]
    LowerCaseLettersError,
    #[msg("Team captain password is not matching")]
    TeamCaptainPasswordError,
    #[msg("Pubkey length is not 32")]
    PubkeyLengthError,
    #[msg("All team members accept the voting")]
    AcceptVotingError,
}




// Instructions

#[derive(Accounts)]
pub struct CreateTeam<'info> {

    #[account(mut,constraint = owner_account.team_size as usize == owner_account.team_players.len() )]
    pub owner_account: Account<'info, Team>,
    #[account(mut)]
    pub user: Signer<'info>,
}


#[derive(Accounts)]
pub struct CreateCaptain<'info> {

    #[account(init, 
        seeds = ["team_captain".as_ref()],
        bump,
        payer = team_captain,space = 8+100,
        constraint = owner_account.team_size as usize == owner_account.team_players.len() )]
    pub owner_account: Account<'info, Team>,
    #[account(mut)]
    pub team_captain: Signer<'info>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct GeneralData<'info> {

    #[account(mut,constraint = owner_account.team_size as usize == owner_account.team_players.len() )]
    pub owner_account: Account<'info, Team>,
    #[account(mut,
    constraint = owner_account.team_players.len() == tournement_account.prize_dist.len())]
    pub tournement_account: Account<'info, Tournament>,
    #[account(mut)]
    pub user: Signer<'info>,
}








//Data part

#[account]
pub struct Team{
    pub team_captain_account:Pubkey,
    pub team_captain_password: String,
    pub team_size: u8,
    pub team_pub_keys: Vec<Pubkey>,
    pub team_players: Vec<String>,
    pub team_votes: Vec<bool>,
    pub team_captain: String,
    pub team_prize: u128,
    pub team_accepted: Vec<bool>
    pub bump: u8,
}

#[account]
pub struct Tournament{
    voted_result: bool,
    prize_dist: Vec<u8>,
    transfer_account: Pubkey
    prize_distrubed:bool
}


