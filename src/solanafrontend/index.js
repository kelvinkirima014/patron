import Wallet from "@project-serum/sol-wallet-adapter";
import {
    Connection,
    SystemProgram,
    Transaction,
    PublicKey,
    TransactionInstruction
} from "@solana/web3.js";
import { deserialize, serialize } from "borsh";

const cluster = "https://api.devnet.solana.com";
const connection = new Connection(cluster, "cluster");
const wallet = new Wallet("https://www.sollet.io", cluster);
const programId = new PublicKey(
	"3LyMjj91nSSQcmNA7h9RZ2N2sb74w4jMx67tRoTCUrT7"
);

export async function setPayerAndBlockhashTransaction(instructions){
	const transaction = new Transaction();
	instructions.forEach(element => {
		transaction.add(element);
	});

	transaction.feePayer = wallet.publicKey;
	let hash = await connection.getLatestBlockhash();
	transaction.recentBlockhash = hash.blockhash;
	return transaction;
}