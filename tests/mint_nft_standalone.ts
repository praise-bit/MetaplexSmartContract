import * as anchor from "@project-serum/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";

async function main() {
  // Anchor provider
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Dummy program ID (matches Anchor.toml)
  const programId = new PublicKey(
    "4Nd1mYw7E7w9G8bXJv2qkZx5MhfV1LQ7KJ2r7s8k9vUe"
  );

  // Generate random mint keypair
  const mintKeypair = Keypair.generate();

  // Generate dummy metadata PDA
  const tokenMetadataProgramId = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );
  const [metadataPDA] = await PublicKey.findProgramAddress(
    [
      Buffer.from("metadata"),
      tokenMetadataProgramId.toBuffer(),
      mintKeypair.publicKey.toBuffer(),
    ],
    tokenMetadataProgramId
  );

  console.log("Mint Key:", mintKeypair.publicKey.toBase58());
  console.log("Metadata PDA:", metadataPDA.toBase58());
  console.log("Program ID:", programId.toBase58());

  console.log("NFT mint simulation complete (no errors).");
}

main().catch((err) => {
  console.error(err);
});
