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

const contractAddress = process.env.CONTRACT_ADDRESS
const client = await ArchwayClient.connect('https://rpc.constantine.archway.io');

//profile
async function profile() {  
    try {
    const msg = {
        profile: {
            id:  accounts[0].address
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

//sigle job
async function sigleJob() {  
    try {
    const msg = {
        single_job: {
            job_id:  1
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

//many job
async function manyJob() {  
    try {
    const msg = {
        many_job: {
            start_after: 0,
            limit: 50
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


//customer job
async function customerJob() {  
    try {
    const msg = {
        customer_job: {
            account_id:  "archway1jphqvc6pa7g4tnjpxznsn3nhzegj9fm090a5tr",
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


//contractor job
async function contratorJob() {  
    try {
    const msg = {
        contractor_job: {
            account_id:  "archway1jphqvc6pa7g4tnjpxznsn3nhzegj9fm090a5tr",
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


//review
async function review() {  
    try {
    const msg = {
        review: {
            job_id:  1,
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

//users
async function users() {  
    try {
    const msg = {
        users: {
            start_after: 0,
            limit: 50
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



// profile()
// sigleJob()
// manyJob()
// customerJob()
// contratorJob()
// review()
users()