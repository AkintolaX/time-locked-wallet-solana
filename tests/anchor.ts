import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";

describe("time-locked-wallet", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.TimeLockedWallet;

  it("Creates a time lock", async () => {
    const amount = new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL);
    const unlockTimestamp = new anchor.BN(Math.floor(Date.now() / 1000) + 120); // 2 minutes
    
    const [timeLockedWalletPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("time_locked_wallet"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    try {
      await program.methods
        .initializeLock(amount, unlockTimestamp)
        .accounts({
          owner: provider.wallet.publicKey,
          timeLockedWallet: timeLockedWalletPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const account = await program.account.timeLockedWallet.fetch(timeLockedWalletPda);
      
      // Simple assertions without chai
      if (account.amount.toString() !== amount.toString()) {
        throw new Error(`Amount mismatch: expected ${amount.toString()}, got ${account.amount.toString()}`);
      }
      
      if (account.owner.toString() !== provider.wallet.publicKey.toString()) {
        throw new Error(`Owner mismatch: expected ${provider.wallet.publicKey.toString()}, got ${account.owner.toString()}`);
      }
      
      console.log("Time lock created successfully!");
      console.log(`Amount locked: ${account.amount.toString()} lamports`);
      console.log(`Unlock time: ${new Date(account.unlockTimestamp.toNumber() * 1000)}`);
      
    } catch (error) {
      console.error("Test failed:", error);
      throw error;
    }
  });

  it("Fails to withdraw before unlock time", async () => {
    const [timeLockedWalletPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("time_locked_wallet"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    try {
      await program.methods
        .withdraw()
        .accounts({
          owner: provider.wallet.publicKey,
          timeLockedWallet: timeLockedWalletPda,
        })
        .rpc();
      
      throw new Error("Should have failed to withdraw early");
    } catch (error) {
      if (error.message.includes("FundsStillLocked")) {
        console.log("Early withdrawal correctly blocked!");
      } else {
        console.error("Unexpected error:", error);
        throw error;
      }
    }
  });
});