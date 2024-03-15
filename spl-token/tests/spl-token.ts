import * as anchor from "@coral-xyz/anchor";
import { SplToken } from "../target/types/spl_token";
import {
  PublicKey,
  Keypair,
  SystemProgram,
} from "@solana/web3.js";
import {
  getAssociatedTokenAddressSync,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("Test deposit and withdraw", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.SplToken as anchor.Program<SplToken>;

  // the mint account that admin want to set
  const mintAccountAddr = new PublicKey("");
  const [tokenPoolAddr, _] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("token_pool"),
      payer.publicKey.toBuffer(),
      mintAccountAddr.toBuffer(),
    ],
    program.programId
  );
  console.log(`tokenPoolAddr address: ${tokenPoolAddr}`);

  const tokenVaultAddr = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), payer.publicKey.toBuffer()],
    program.programId
  )[0];
  console.log(`tokenVaultAddr address: ${tokenVaultAddr}`);
  const vaultAtaAddr = getAssociatedTokenAddressSync(
    mintAccountAddr,
    tokenVaultAddr,
    true
  );
  console.log(`vaultAtaAddr address: ${vaultAtaAddr}`);

  it("Set tokens", async () => {
    try {
      const txSig = await program.methods
        .set()
        .accounts({
          admin: payer.publicKey,
          mintAccount: mintAccountAddr,
          tokenPool: tokenPoolAddr,
          tokenVault: tokenVaultAddr,
          vaultAta: vaultAtaAddr,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([payer.payer])
        .rpc();
      console.log("Set successfully!");
      console.log(`Transaction Signature: ${txSig}`);
    } catch (err) {
      console.log("Failed to set token");
      console.log(err);
    }
  });

  // use specified test account that already hold specified SPL tokens
  let secretKey = new Uint8Array([]);
  let kp = Keypair.fromSecretKey(secretKey);
  console.log(`kp address: ${kp.publicKey}`);
  const userInfoAddr = PublicKey.findProgramAddressSync(
    [
      Buffer.from("user_info"),
      kp.publicKey.toBuffer(),
      mintAccountAddr.toBuffer(),
    ],
    program.programId
  )[0];
  console.log(`userInfoAddr address: ${userInfoAddr}`);
  const depositorAta = getAssociatedTokenAddressSync(
    mintAccountAddr,
    kp.publicKey
  );
  console.log(`depositorAta address: ${depositorAta}`);

  it("Add user", async () => {
    try {
      const txSig = await program.methods
        .addUser()
        .accounts({
          depositor: kp.publicKey,
          mintAccount: mintAccountAddr,
          tokenPool: tokenPoolAddr,
          userInfo: userInfoAddr,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([kp])
        .rpc();
      console.log("Add user successfully!");
      console.log(`Transaction Signature: ${txSig}`);
    } catch (err) {
      console.log("Failed to add user");
      console.log(err);
    }
  });

  it("Deposit tokens", async () => {
    const amount = new anchor.BN(100);
    try {
      const txSig = await program.methods
        .deposit(amount)
        .accounts({
          depositor: kp.publicKey,
          tokenPool: tokenPoolAddr,
          mintAccount: mintAccountAddr,
          userInfo: userInfoAddr,
          tokenVault: tokenVaultAddr,
          vaultAta: vaultAtaAddr,
          depositorTokenAccount: depositorAta,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([kp])
        .rpc();
      console.log("Deposit successfully!");
      console.log(`Transaction Signature: ${txSig}`);
    } catch (err) {
      console.log("Failed to deposit token");
      console.log(err);
    }
  });

  it("Withdraw tokens", async () => {
    const amount = new anchor.BN(10);
    try {
      const txSig = await program.methods
        .withdraw(amount)
        .accounts({
          user: kp.publicKey,
          tokenPool: tokenPoolAddr,
          mintAccount: mintAccountAddr,
          userInfo: userInfoAddr,
          tokenVault: tokenVaultAddr,
          vaultAta: vaultAtaAddr,
          recipientTokenAccount: depositorAta,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([kp])
        .rpc();
      console.log("Withdraw successfully!");
      console.log(`Transaction Signature: ${txSig}`);
    } catch (err) {
      console.log("Failed to withdraw token");
      console.log(err);
    }
  });
});
