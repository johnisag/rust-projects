import * as anchor from "@coral-xyz/anchor";
import * as web3 from "@solana/web3.js";
import type { Counter } from "../target/types/counter";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.Counter as anchor.Program<Counter>;

const wallet = pg.wallet;
const program = program;
const counterSeed = Buffer.from("counter");

const counterPubkey = await web3.PublicKey.findProgramAddressSync(
  [counterSeed],
  program.programId
);

const initializeTx = await program.methods
  .initialize()
  .accounts({
    counter: counterPubkey[0],
    authority: program.provider.publicKey,
    systemProgram: web3.SystemProgram.programId,
  })
  .rpc();

let counterAccount = await program.account.counter.fetch(counterPubkey[0]);
console.log("account after initializing ==> ", Number(counterAccount.count));

const incrementTx = await program.methods
  .increment()
  .accounts({
    counter: counterPubkey[0],
    authority: program.provider.publicKey,
  })
  .rpc();

counterAccount = await program.account.counter.fetch(counterPubkey[0]);
console.log("account after increasing ==>", Number(counterAccount.count));