import { web3 } from "@project-serum/anchor";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";

// for rest function
import {
  TOKEN_PROGRAM_ID,
  AccountLayout,
  MintLayout,
  createMint,
  createAccount,
  mintTo,
} from "@solana/spl-token";

let bs58 = require("bs58");
import BN from "bn.js";
import {
  Data,
  updateMetadata,
  Creator,
  createMetadata,
  createMasterEdition,
  getMetadata,
} from "./metadata";
import { User } from "../config/users";

export const mintNewNFT = async (
  creator: User,
  owner: User
): Promise<Array<PublicKey>> => {
  // Create new token mint
  const newMintKey = await createMint(
    creator.provider.connection,
    creator.keypair,
    creator.publicKey,
    null,
    0
  );
  const nftAccount = await createAccount(
    creator.provider.connection,
    owner.keypair,
    newMintKey,
    owner.publicKey
  );
  await mintTo(
    creator.provider.connection,
    owner.keypair,
    newMintKey,
    nftAccount,
    creator.keypair,
    1
  );

  const name = "Test NFT";
  const metadataUrl = "https://metaverse-nft/testnft01";

  const creators = [
    new Creator({
      address: creator.publicKey.toBase58(),
      share: 37,
      verified: true,
    }),
    new Creator({
      address: "7diGCKfWSnqujiC9GvK3mpwsF5421644SbDEHKtSho1d",
      share: 60,
      verified: false,
    }),
    new Creator({
      address: "GC9Ln3MRWahCrgjdtRANZyF5vpVd9XWgJibJsuNUXWLB",
      share: 3,
      verified: false,
    }),
  ];

  let data = new Data({
    name: name,
    symbol: "MTVS",
    uri: metadataUrl,
    creators,
    sellerFeeBasisPoints: 800,
  });

  let instructions: TransactionInstruction[] = [];

  await createMetadata(
    data,
    creator.publicKey.toBase58(),
    newMintKey.toBase58(),
    creator.publicKey.toBase58(),
    instructions,
    creator.publicKey.toBase58()
  );

  await createMasterEdition(
    new BN(1),
    newMintKey.toBase58(),
    creator.publicKey.toBase58(),
    creator.publicKey.toBase58(),
    creator.publicKey.toBase58(),
    instructions
  );
  const transaction = new Transaction();
  transaction.add(...instructions);
  let txHash = await sendAndConfirmTransaction(
    creator.provider.connection,
    transaction,
    [creator.keypair]
  );
  return [nftAccount, newMintKey];
};
