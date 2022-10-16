import {Program, AnchorProvider} from '@project-serum/anchor';
import { useConnection,useWallet as useAdapterWallet,useAnchorWallet,WalletContextState,AnchorWallet} from '@solana/wallet-adapter-react';
import { Connection,PublicKey } from '@solana/web3.js';

const idl = '../../../target/idl/team_dao_voting_program.json'
const programId = new anchor.web3.PublicKey("4tcEoG3qEP4LHkdboUCJvusV4hJidiFAtz1knfngwMFd");

export const createDataHook = (
    connection,
    adapterWalletObj,
    anchorWalletObj,
    provider,
    program,
 ) => {
    const {connection} = useConnection();
    const adapterWalletObj = useAdapterWallet();
    const anchorWalletObkj = useAnchorWallet();

    provider = new AnchorProvider(connection,adapterWalletObj,{});
    const program = new Program(idl,programId,provider);

    return{
        connection,
        adapterWalletObj,
        anchorWalletObj,
        provider,
        program,
    }
};