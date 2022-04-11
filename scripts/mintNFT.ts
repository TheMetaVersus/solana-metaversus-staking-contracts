/* imageList
https://arweave.net/fOxOGl5ZrR3A5cb_L8JTTIUIhDGXya1HmA8nM1WCsfk
https://arweave.net/jlqvyASco_nk54z3dKeOvK3EHNRgDXvkrDzPDpwLx7I
https://arweave.net/8SDQOaXMzHWobCX5J1FFwdIFPT-ulU6Q4PavpnUU2EI
https://arweave.net/pakDHI1dK4wK13Q-nQ1Rz6tMuE1PVfr_DkvDfnGd67g
https://arweave.net/-k-8fFNN5zLfWQWjg69cbvyT3HrN3ItgJnssgTJj-tg
https://arweave.net/1u9Sa4XiLvQ9tFp_O_yZLTOh4MylEo-nOrg4Ktd11kE
https://arweave.net/Ujs_amdPrXgZjaM1qs9S4RPK_32E6pnQuzz9-738-7g
https://arweave.net/OYG8sUjJva4hqTLaPHGp2rGm0382HYsi4lnry3UssCk
https://arweave.net/6SIznTEuYPwHLQanDLuyEDRIAD2F48X1uSf8EH16UtI
*/
/*
https://arweave.net/gB_r0AKBDkHcsblKVdVGuSjQ8pO04koB0euihkkNNFI
https://arweave.net/x33a2PrgXcdOYCcSUydMoHTnnUjsb5PZ_A3D5sp7LQY
https://arweave.net/LZsjDwXpfTii1S3gultOkzpHx7L0g2mnYLgheMXr0kM
https://arweave.net/79tqEVK916H-IrXGVrjKoqWOhnzDXYAKw101luVctRg
https://arweave.net/yP2bQCgI4tyLgBJMtpxXrWuUSt0uCcI9NR5s67HSjDo
https://arweave.net/RpuJLV5wmhuR2WuVPLhBDupArtySg-GreN9YmyHma40
https://arweave.net/8QNsiFj-og21_KTJUsqjaFnuh_VexYFnYmem0mw5U20
https://arweave.net/uVaO5z4fWESk61zhO1Mbz44fUymwFTGOnLdx45izeq8
https://arweave.net/yIj7fGXwPeBWnXNctfyWpKi7nqwpFgqDLgEyMikBBv4
*/
import { web3 } from '@project-serum/anchor';
import { Keypair, 
        PublicKey, 
        SystemProgram,
        Transaction,
        TransactionInstruction,
        sendAndConfirmTransaction,
        Connection,
        clusterApiUrl
       } from '@solana/web3.js';

// for rest function
import {
  TOKEN_PROGRAM_ID, 
  AccountLayout, 
  MintLayout,
  createMint,
  createAccount,
  mintTo
} from "@solana/spl-token";

let bs58 = require('bs58');
import BN from 'bn.js';
import { Data, updateMetadata, Creator, createMetadata, createMasterEdition, getMetadata } from '../tests/nft/metadata';

const creator: Keypair = Keypair.fromSecretKey(bs58.decode("5cMBJTq18Bvsrsv4wZDA7J2smHbDPWgRsnLrFc2x5ZbeWrSfX2Ltftsw2Uc87ff5sfHa2iPfZMB9ch2117JyX7ki"));
const connection = new Connection(clusterApiUrl("devnet"));

const mintNewNFT = async (metadataUrl: string) : Promise<Array<PublicKey>>  => {
  // Create new token mint
  const newMintKey = await createMint(connection, creator, creator.publicKey, null, 0);
  console.log("mint created");
  const nftAccount = await createAccount(connection, creator, newMintKey, creator.publicKey);
  console.log("account created");
  await mintTo(connection, creator, newMintKey, nftAccount, creator, 1);

  const name = "Metaversus Test NFT";

  const creators = [
    new Creator({
        address: creator.publicKey.toBase58(),
        share: 100,
        verified: true
    }),
    new Creator({
      address: 'JAwNgkoSRMJzMndLtxBVSVp3ZPUfw1MEJ5GaAQ2gWcDT',
      share: 0,
      verified: false
    }),
    new Creator({
      address: '36gJMRpN2dTyYegNBtTa5RvndhWr7vPL91E7hV5zcQKA',
      share: 0,
      verified: false
    }),
  ];
  
  let data = new Data({
    name: name,
    symbol: "MTVNFT",
    uri: metadataUrl,
    creators,
    sellerFeeBasisPoints: 800
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
  console.log("metadata created");
  await createMasterEdition(
    new BN(1),
    newMintKey.toBase58(),
    creator.publicKey.toBase58(),
    creator.publicKey.toBase58(),
    creator.publicKey.toBase58(),
    instructions
  );
  console.log("masteredition created");
  const transaction = new Transaction();
  transaction.add(...instructions);
  let txHash = await sendAndConfirmTransaction(
    connection,
    transaction,
    [creator],
  );
  console.log(txHash);
  return [
    nftAccount, newMintKey
  ];
};

async function main() {
  let metaList = [
    "https://arweave.net/gB_r0AKBDkHcsblKVdVGuSjQ8pO04koB0euihkkNNFI",
    "https://arweave.net/x33a2PrgXcdOYCcSUydMoHTnnUjsb5PZ_A3D5sp7LQY",
    "https://arweave.net/LZsjDwXpfTii1S3gultOkzpHx7L0g2mnYLgheMXr0kM",
    "https://arweave.net/79tqEVK916H-IrXGVrjKoqWOhnzDXYAKw101luVctRg",
    "https://arweave.net/yP2bQCgI4tyLgBJMtpxXrWuUSt0uCcI9NR5s67HSjDo",
    "https://arweave.net/RpuJLV5wmhuR2WuVPLhBDupArtySg-GreN9YmyHma40",
    "https://arweave.net/8QNsiFj-og21_KTJUsqjaFnuh_VexYFnYmem0mw5U20",
    "https://arweave.net/uVaO5z4fWESk61zhO1Mbz44fUymwFTGOnLdx45izeq8",
    "https://arweave.net/yIj7fGXwPeBWnXNctfyWpKi7nqwpFgqDLgEyMikBBv4"
  ];
  metaList.forEach(async meta => {
    await mintNewNFT(meta);
  })
}
main();