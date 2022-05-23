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

export async function signAndSendTransaction(transaction){
	try{
		console.log("signAndSendTransaction");
		let signedTransaction = await wallet.signTransaction(transaction);
		console.log("signed transaction");
		let signature = await connection.sendRawTransaction(
			signedTransaction.serialize()
		);
		console.log("end sign and send transaction");
		return signature;
	} catch(e){
		console.log("failure signing and sending transaction"), e;
		throw e;

	}
}

class CampaignDetails{
	constructor(properties){
		object.keys(properties).forEach(key =>{
			this[key] = properties[key];
		});
	}
		static schema = new Map([[CampaignDetails,
			{
				kind: 'struct',
				fields: [
					['admin', [32]],
					['name', 'string'],
					['description', 'string'],
					['image_link', 'string'],
					['amount_donated', 'u64']
				]
			}	
		]]);	
}

export async function createCampaign(
	name, description, image_link
){
	await checkWallet();

}

async function checkWallet() {
		if (!wallet.connected()){
			await wallet.connect();
		}
}

const SEED = "abcdef" + Math.random().toString();
    let newAccount = await PublicKey.createWithSeed(
        wallet.publicKey,
        SEED,
        programId
    );

let campaign =new CampaignDetails({
        name: String,
        description: description,
        image_link: image_link,
        admin: wallet.publicKey.toBuffer(),
        amount_donated: 0
    })