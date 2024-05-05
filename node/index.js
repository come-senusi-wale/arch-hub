import { SigningArchwayClient, ArchwayClient } from '@archwayhq/arch3.js';
import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import fs from 'fs';
import * as base64js from "base64-js";
import dotenv from "dotenv";
dotenv.config();

 // Set up the network data:
 const network = {
    chainId: 'constantine-3',
    endpoint: 'https://rpc.constantine.archway.io',
    prefix: 'archway',
};

console.log(1)

// Set up your wallet using your mnemonic:
const mnemonic = process.env.MNEMONIC;
const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, { prefix: network.prefix });
const accounts = await wallet.getAccounts();

// Create the SigningArchwayClient using connectWithSigner method.
const signingClient = await SigningArchwayClient.connectWithSigner(network.endpoint, wallet);

console.log(4)


async function main() {

    try {

        // Set up the network data:
        // const network = {
        //     chainId: 'constantine-3',
        //     endpoint: 'https://rpc.constantine.archway.io',
        //     prefix: 'archway',
        // };

        // console.log(1)

        // // Set up your wallet using your mnemonic:
        // const mnemonic = process.env.MNEMONIC;
        // const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, { prefix: network.prefix });
        // const accounts = await wallet.getAccounts();

        console.log(2)
        
        // Set account and beneficiary addresses:
        const accountAddress = accounts[0].address;
        const beneficiaryAddress = process.env.BENEFICIARY_ADDRESS;

        console.log(3)

        // Create the SigningArchwayClient using connectWithSigner method.
        // const signingClient = await SigningArchwayClient.connectWithSigner(network.endpoint, wallet);

        console.log(4)

        // Perform the following steps to read the file, encode it to base64, and convert it to a ByteArray:
        const wasmCode = fs.readFileSync('../target/wasm32-unknown-unknown/release/arch_hub.wasm');
        const encoded = Buffer.from(wasmCode, 'binary').toString('base64');
        const contractData = base64js.toByteArray(encoded);

        console.log(5)

        // Broadcast and sign the transaction with the signing client:
        const uploadResult = await signingClient.upload(
            accountAddress,
            contractData,
            'auto',
            '',
        );

        console.log(6)

        if (uploadResult.code !== undefined && uploadResult.code !== 0) {
            console.log("Storage failed:", uploadResult.log || uploadResult.rawLog);

            console.log(7)
        } else {
            console.log("Storage successful:", uploadResult.transactionHash);

            console.log(8)
        }

        const codeId = uploadResult.codeId;
        console.log('codeId', codeId)

        console.log(9)

        // Create a msg object to instantiate the contract. You can also add some instantiation options:
        const msg = {
            // verifier: accountAddress, 
            // beneficiary: beneficiaryAddress,
        };

        console.log(10)

        const instantiateOptions = {
            memo: "Instantiating a new contract",
            funds: [
                {
                denom: 'aconst',
                amount: '1060000000000000000'
                }
            ],
            admin: accounts[0].address
        };

        console.log(11)

        // Broadcast and sign the transaction with the signing client:
        const instantiateResult = await signingClient.instantiate(
            accountAddress,
            codeId,
            msg,
            'my-instance-label',
            'auto',
            instantiateOptions
        );

        console.log(12)

        // Verify if the transaction was successful by checking the broadcastResult2. This is the final lines of code for fucntion main:
        if (instantiateResult.code !== undefined && instantiateResult.code !== 0) {
            console.log(13)
            console.log("Instantiation failed:", instantiateResult.log || instantiateResult.rawLog);
            console.log(14)
        } else {
            console.log(15)
            console.log("Instantiation successful:", instantiateResult.transactionHash);
            console.log('initaiateContractResult', instantiateResult)
            console.log(16)
        }

        console.log(17)
        
    } catch (error) {
        console.log(' occure', error)
    }

    
      
}

// main()

const contractAddress = 'archway1yd3xfkyktp7sty3806cdugnfpyykfj7mkly3wt7ux7m6x7v7r6fsrr4lwr'
const client = await ArchwayClient.connect('https://rpc.constantine.archway.io');

//query 
async function queryContract() {
    
    try {
    const msg = {
        profile: {
            id:  "waledayofiv.arch"
        },
    };

    const query = await client.queryContractSmart(
        contractAddress,
        msg
    );

    console.log("query: ", query);

    } catch (error) {
        console.log('error', error)
    }
}

async function signContract() {
    let cost = '1000000000000000000'
    
    try {
    
    const msg = {
        create_profile: {
            name: "waledayofiv",
            hour_rate: "100000",
            cost: cost
        }



        // update_hourly_rate: {
        //     name: "waledayotre.arch",
        //     hour_rate: "5000",    
        // }


        // set_availability: {
        //     name: "waledayotre.arch",
        //     available: true,  
        // }


        // update_metadata: {
        //     name: "waledayofor",
        //     update: {
        //         description: "I'm in love with Zainab",
        //         // Other properties stay the same
        //         image: "ipfs://QmNoMUgTM82EGaTCTnuEUJDusV21UEGSgKM5RhM1C9N3WE",
        //         accounts: [{username: "archid-protocol",profile: "https://github.com/archid-protocol",account_type: "github",verfication_hash: null}],
        //         websites: [{url: "https://archid.app",domain: "dapp.archid.arch",verfication_hash: null}]
        //     },  
        // }

    };

    let funds = [{
        denom: 'aconst',
        amount: cost,
    }]

    const signContract = await signingClient.execute(
        accounts[0].address,
        contractAddress,
        msg,
        "auto",
        "Registering domain",
        funds
    );
      

    console.log("signContract: ", signContract);

    } catch (error) {
        console.log('error', error)
    }
}

queryContract()
// signContract()