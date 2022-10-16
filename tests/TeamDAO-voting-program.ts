import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { publicKey } from "@project-serum/anchor/dist/cjs/utils";
import { expect } from "chai";
import { create } from "domain";
import { TeamDaoVotingProgram } from "../target/types/team_dao_voting_program";

describe("TeamDAO-voting-program", () => {
  const providerAnchor = anchor.AnchorProvider.env()
  anchor.setProvider(providerAnchor)
  const user = providerAnchor.wallet
  const program = anchor.workspace.TeamDaoVotingProgram as Program<TeamDaoVotingProgram>;

  

  let ownerAccount = anchor.web3.Keypair.generate();

  // there is a problem about testing using windows machine

  // so I know how to write to tests but my devops skills are low.
  // I couldn't configure my localhost balance is 0

  it('Team is created', async () => {
    const tx = await program.rpc.createTeam(3,["martin","george","blake"],"blake",{
      accounts:{
        ownerAccount:ownerAccount.publicKey,
        user:user.publicKey,
      },
    }).catch((err) => console.log(err))
    const storedInfo = await program.account.team.fetch(ownerAccount.publicKey)
    expect(storedInfo.teamCaptain).to.equal("blake")
    expect(storedInfo.teamSize).to.equal(3);
    expect(storedInfo.teamPlayers).to.equal(["martin","george","blake"]);

    console.log("Your transaction signature", tx);
  });

  it("Captain is created", async () => {
    const tx = await program.rpc.createCaptain("martin",{
      accounts:{
        ownerAccount:ownerAccount.publicKey,
        teamCaptain:user.publicKey,
        systemProgram:anchor.web3.SystemProgram.programId,
      },
    }).catch((err) =>console.log(err))
    const storedInfo = await program.account.team.fetch(ownerAccount.publicKey)
    expect(storedInfo.teamCaptain).to.equal("martin")
    console.log("Your transaction signature",tx)
  })


  it("Get votes", async () => {
    const tx = await program.rpc.getVotes([true,false,true,false,true],{
      accounts:{
        ownerAccount:ownerAccount.publicKey,
        tournementAccount:user.publicKey,
        user:user.publicKey,
      },
    }
    ).catch((err) =>console.log(err))
    const storedInfo = await program.account.team.fetch(ownerAccount.publicKey)
    expect(storedInfo.votedResult).to.equal([true,false,true,false,true])
    console.log("Your transaction signature",tx)
  })

  // an similarly checking the system

  


  


  // I didn't update functions there is an error on requests

  it("Votes Calculated", async () => {
    const tx = await program.rpc.votesCalculate({
      accounts:{
        ownerAccount:ownerAccount.publicKey,
        tournementAccount:user.publicKey,
        user:user.publicKey,
      }
    }
    ).catch((err) =>console.log(err))
    console.log("Your transaction signature",tx)
  })


  
  it("Join to Team", async () => {
    const tx = await program.rpc.joinToTeam(["jennifer","aniston","huston"],{
      accounts:{
        ownerAccount:ownerAccount.publicKey,
        tournementAccount:user.publicKey,
        user:user.publicKey,
      }
    }
    ).catch((err) =>console.log(err))
    console.log("Your transaction signature",tx)
  })


  it("Leave from the team", async () => {
    const tx = await program.rpc.joinToTeam(["jennifer","huston"],{
      accounts:{
        ownerAccount:ownerAccount.publicKey,
        tournementAccount:user.publicKey,
        user:user.publicKey,
      }
    }
    ).catch((err) =>console.log(err))
    console.log("Your transaction signature",tx)
  })

  it("Prize calculate", async () => {
    const tx = await program.rpc.prizeCalculate([12,32,49,12,12],{
      accounts:{
        ownerAccount:ownerAccount.publicKey,
        tournementAccount:user.publicKey,
        user:user.publicKey,
      }
    }
    ).catch((err) =>console.log(err))
    console.log("Your transaction signature",tx)
  })


  it("Create Captain Password", async () => {
    const tx = await program.rpc.createCaptainPassword("9as8das'LR-!",{
      accounts:{
        ownerAccount:ownerAccount.publicKey,
        teamCaptain:user.publicKey,
        systemProgram:anchor.web3.SystemProgram.programId,
      },
    }
    ).catch((err) =>console.log(err))
    console.log("Your transaction signature",tx)
  })



  it("Get Team Pubkeys", async () => {
    
    const tx = await program.rpc.getPubkeys([user.publicKey,user.publicKey,user.publicKey],{
      accounts:{
        ownerAccount:ownerAccount.publicKey,
        tournementAccount:user.publicKey,
        user:user.publicKey,
      }
    }
    ).catch((err) =>console.log(err))
    console.log("Your transaction signature",tx)
  })

  it("Transfer sol to captain", async () => {
    
    const tx = await program.rpc.transferSolToCaptain("9as8das'LR-!",{
      accounts:{
        ownerAccount:ownerAccount.publicKey,
        tournementAccount:user.publicKey,
        user:user.publicKey,
      }
    }
    ).catch((err) =>console.log(err))
    console.log("Your transaction signature",tx)
  })

  it("Distribute sol to team", async () => {
    //getting and couldn't fix it Error: failed to get recent blockhash: FetchError: request to http://localhost:8899/ failed, reason: connect ECONNREFUSED 127.0.0.1:8899
    const tx = await program.rpc.transferNativeSol("9as8das'LR-!",{
      accounts:{
        ownerAccount:ownerAccount.publicKey,
        tournementAccount:user.publicKey,
        user:user.publicKey,
      }
    }
    ).catch((err) =>console.log(err))
    console.log("Your transaction signature",tx)
  })






})