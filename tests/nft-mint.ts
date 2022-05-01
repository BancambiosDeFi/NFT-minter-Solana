import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NftMint } from "../target/types/nft_mint";
import {
  TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAddress,
  createInitializeMintInstruction,
  MINT_SIZE,
} from "@solana/spl-token"; // IGNORE THESE ERRORS IF ANY
import {Connection, PublicKey} from "@solana/web3.js"

async function getOrCreateAssociatedTokenAccount(
  connection: Connection,
  mint: PublicKey,
  wallet: PublicKey
): Promise<PublicKey> {
  const associatedTokenAddress = await getAssociatedTokenAddress(mint, wallet)
  if (await connection.getAccountInfo(associatedTokenAddress)) {
      return associatedTokenAddress
  }
  console.log("create associated token account for", wallet.toBase58())
  return;
}

async function main() {
const { PublicKey, SystemProgram } = anchor.web3;
  // Configure the client to use the local cluster.
  require("dotenv").config(); 
  anchor.setProvider(anchor.Provider.env());
  const program = anchor.workspace
    .NftMint as Program<NftMint>;
    // Add your test here.
    const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
      "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
    );
    const lamports: number =
      await program.provider.connection.getMinimumBalanceForRentExemption(
        MINT_SIZE
      );
    const getMetadata = async (
      mint: anchor.web3.PublicKey
    ): Promise<anchor.web3.PublicKey> => {
      return (
        await anchor.web3.PublicKey.findProgramAddress(
          [
            Buffer.from("metadata"),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
          ],
          TOKEN_METADATA_PROGRAM_ID
        )
      )[0];
    };

    const getMasterEdition = async (
      mint: anchor.web3.PublicKey
    ): Promise<anchor.web3.PublicKey> => {
      return (
        await anchor.web3.PublicKey.findProgramAddress(
          [
            Buffer.from("metadata"),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
            Buffer.from("edition"),
          ],
          TOKEN_METADATA_PROGRAM_ID
        )
      )[0];
    };
    const mintKey: anchor.web3.Keypair = anchor.web3.Keypair.generate();
    
    const NftTokenAccount = await getAssociatedTokenAddress(
      mintKey.publicKey,
      program.provider.wallet.publicKey
    );
    console.log("NFT Account: ", NftTokenAccount.toBase58());

    const mint_tx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: program.provider.wallet.publicKey,
        newAccountPubkey: mintKey.publicKey,
        space: MINT_SIZE,
        programId: TOKEN_PROGRAM_ID,
        lamports,
      }),
      createInitializeMintInstruction(
        mintKey.publicKey,
        0,
        program.provider.wallet.publicKey,
        program.provider.wallet.publicKey
      ),
      createAssociatedTokenAccountInstruction(
        program.provider.wallet.publicKey,
        NftTokenAccount,
        program.provider.wallet.publicKey,
        mintKey.publicKey
      )
    );
    const res = await program.provider.send(mint_tx, [mintKey]);
    console.log(
      await program.provider.connection.getParsedAccountInfo(mintKey.publicKey)
    );

    console.log("Account: ", res);
    console.log("Mint key: ", mintKey.publicKey.toString());
    console.log("User: ", program.provider.wallet.publicKey.toString());

    const metadataAddress = await getMetadata(mintKey.publicKey);
    const masterEdition = await getMasterEdition(mintKey.publicKey);

    console.log("Metadata address: ", metadataAddress.toBase58());
    console.log("MasterEdition: ", masterEdition.toBase58());
    console.log("===rpc========",program.rpc.mintNft);
    const connection = new Connection("https://api.devnet.solana.com", "confirmed")
    const userTokenPubkey = await getOrCreateAssociatedTokenAccount(connection, 
        new anchor.web3.PublicKey("CZyEKArwVYSKkv9im3grGNXmggbPfS8YGUovBnzoKQ4s"), // test USDT
        new anchor.web3.PublicKey("CD6To88A4KrApbnDUkHrwpjMY5ufgPpVQzm9rRX5d3ro") 
    );
    console.log("userTokenPubkey", userTokenPubkey.toBase58())
    console.log("walletAddress", program.provider.wallet.publicKey.toBase58())
    const tx = await program.rpc.mintNft(
      mintKey.publicKey,
      "https://arweave.net/y5e5DJsiwH0s_ayfMwYk-SnrZtVZzHLQDSTZ5dNRUHA",
      "NFT Title",
      {
        accounts: {
          mintAuthority: program.provider.wallet.publicKey,
          mint: mintKey.publicKey,
          tokenAccount: NftTokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          metadata: metadataAddress,
          tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
          payer: program.provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          masterEdition: masterEdition,
          // walletAddress:  new anchor.web3.PublicKey("CD6To88A4KrApbnDUkHrwpjMY5ufgPpVQzm9rRX5d3ro"),
          // ataAddress: userTokenPubkey,
        },
      },
      
    );
    console.log("Your transaction signature", tx);
}

main()
    .then(() => process.exit(0))
    .catch((err) => console.error(err))
