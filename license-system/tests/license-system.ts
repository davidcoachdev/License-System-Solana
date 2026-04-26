import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LicenseSystem } from "../target/types/license_system";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";

describe("license-system", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.LicenseSystem as Program<LicenseSystem>;
  
  let owner: Keypair;
  let licensePda: PublicKey;

  before(async () => {
    owner = Keypair.generate();
    
    // Airdrop SOL to owner
    const signature = await provider.connection.requestAirdrop(
      owner.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(signature);

    // Derive license PDA
    [licensePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("license"), owner.publicKey.toBuffer()],
      program.programId
    );
  });

  it("Issues a license", async () => {
    const productId = "test-product-001";
    const durationDays = 30;

    const tx = await program.methods
      .issueLicense(owner.publicKey, productId, new anchor.BN(durationDays))
      .accounts({
        license: licensePda,
        authority: owner.publicKey,
        owner: owner.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([owner])
      .rpc();

    console.log("Issue license tx:", tx);

    // Fetch and verify license
    const license = await program.account.license.fetch(licensePda);
    assert.equal(license.owner.toString(), owner.publicKey.toString());
    assert.equal(license.productId, productId);
    assert.equal(license.isRevoked, false);
    assert.isTrue(license.expiresAt.toNumber() > Date.now() / 1000);
  });

  it("Extends a license", async () => {
    const additionalDays = 15;
    
    const licenseBefore = await program.account.license.fetch(licensePda);
    const expiresAtBefore = licenseBefore.expiresAt.toNumber();

    const tx = await program.methods
      .extendLicense(new anchor.BN(additionalDays))
      .accounts({
        license: licensePda,
        authority: owner.publicKey,
      })
      .signers([owner])
      .rpc();

    console.log("Extend license tx:", tx);

    const licenseAfter = await program.account.license.fetch(licensePda);
    const expiresAtAfter = licenseAfter.expiresAt.toNumber();
    
    const expectedIncrease = additionalDays * 24 * 60 * 60;
    assert.approximately(expiresAtAfter - expiresAtBefore, expectedIncrease, 5);
  });

  it("Validates a license", async () => {
    const productId = "test-product-001";

    const result = await program.methods
      .validateLicense(productId)
      .accounts({
        license: licensePda,
      })
      .view();

    console.log("Validate license result:", result);
    assert.isTrue(result);
  });

  it("Revokes a license", async () => {
    const tx = await program.methods
      .revokeLicense()
      .accounts({
        license: licensePda,
        authority: owner.publicKey,
      })
      .signers([owner])
      .rpc();

    console.log("Revoke license tx:", tx);

    const license = await program.account.license.fetch(licensePda);
    assert.equal(license.isRevoked, true);
  });

  it("Fails to validate revoked license", async () => {
    const productId = "test-product-001";

    const result = await program.methods
      .validateLicense(productId)
      .accounts({
        license: licensePda,
      })
      .view();

    console.log("Validate revoked license result:", result);
    assert.isFalse(result);
  });
});
