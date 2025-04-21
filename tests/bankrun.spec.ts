import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey } from '@solana/web3.js';
import { BanksClient, ProgramTestContext, startAnchor } from "solana-bankrun";
import IDL from '../../../../target/idl/vesting.json';
import { program, SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { BankrunProvider } from "anchor-bankrun";
import { Vesting } from '../../../../target/types/vesting';
// import { createMint } from "spl-token-bankrun";
import { createMint } from "@solana/spl-token";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";


describe("Vesting smart contract test",()=>{
    
    let beneficary : Keypair;
    let context : ProgramTestContext;
    let provider : BankrunProvider;
    let program: anchor.Program<Vesting>;
    let banksClient: BanksClient;
    let employer : Keypair;
    let mint: PublicKey;
    let beneficaryProivder: BankrunProvider;
    let program2: anchor.Program<Vesting>;
    let vestingAccountKey: PublicKey;
    let treasuryTokenAccount: PublicKey;
    let employeeAccount: PublicKey;
    
    const companyName = "companyName";


    beforeEach(async ()=> {
        beneficary = new anchor.web3.Keypair();

        context = await startAnchor(
            "",
            [{name: "vesting", programId: new PublicKey(IDL.address)}],
            [{
                address: beneficary.publicKey,
                info: {
                    lamports: 1_000_000_000,
                    data: Buffer.alloc(0),
                    owner: SYSTEM_PROGRAM_ID,
                    executable: false
                }
            }]
        );

        provider = new BankrunProvider(context);
        anchor.setProvider(provider);
        program = new anchor.Program<Vesting>(IDL as Vesting, provider);

        banksClient = context.banksClient;
        employer = provider.wallet.payer;

        // @ts-expect-error
        mint = await createMint(banksClient, employer.publicKey, null, null, 2 );
        beneficaryProivder = new BankrunProvider(context);
        beneficaryProivder.wallet = new NodeWallet(beneficary);

        program2 = new anchor.Program<Vesting>(IDL as Vesting, beneficaryProivder);

        [vestingAccountKey] = PublicKey.findProgramAddressSync(
            [Buffer.from(companyName)],
            program.programId
        );

        [treasuryTokenAccount] = PublicKey.findProgramAddressSync(
            [Buffer.from("vesting_treasury"), Buffer.from(companyName)],
            program.programId
        );

        [employeeAccount] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("employee_vesting"),
                beneficary.publicKey.toBuffer(),
                Buffer.from(companyName)
            ],
            program.programId
        );

    });


});