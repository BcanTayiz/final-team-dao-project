# final-team-dao-project

## Functions in the project smart contract

- create team: creating team with player, captain and team size and checking if values correct,
- create bump: creating bump for security and orginality issues,
- create captain: attenting captain for the team from team players,
- create captain password: creating captain password to get and transfer sol, password shall include low, special and similar characters,
- team members accept vote: team members should vote to accept voting before process,
- get votes: getting votes as a vector of boolean,
- votes calculate: calculating votes for majority of 40% and checking before if values are correct and matching,
- join to team: if players is not in the list accept players nickname or username,
- leave from team: first checks players then removes the player from list
- prize calculate: calculating dist u8 values if they are sum is 100,such as [10,20,34,21,15] => 100,
- get pubkeys: getting pubkeys of player's accounts,
- transfer sol to captain: first give all prize to captain,
- transfer native sol: distributes the prize from captain to all team.

## Instructions

Create Team: owner_account mutable account, user is signer
Create Captain: owner_account PDA, team_captain is signer, system_program is system program,
GeneralData: Created for general functionality to get team and tournement accounts

## Accounts

data accounts

//Data part

```
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
```


--------------------------------------------------------------
```
how to run project:
anchor build,
anchor deploy --program-name program-id,
anchor run test
```
