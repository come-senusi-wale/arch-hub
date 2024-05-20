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

// create profile
async function createProfile() {
    let cost = '1000000000000000000'
    
    try {
    
    const msg = {
        create_profile: {
            name: "waledayotwe",
            hour_rate: "100000",
            cost: cost
        }
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


// set hourly rate
async function hourlyRate() {
    try {
        
        const msg = {   
            update_hourly_rate: {
                name: "waledayosev.arch",
                hour_rate: "5000",    
            }
        };

        const signContract = await signingClient.execute(
            accounts[0].address,
            contractAddress,
            msg,
            "auto",
        );
        

        console.log("signContract: ", signContract);

    } catch (error) {
        console.log('error', error)
    }
}


// set availability
async function availability() {
    try {
    
    const msg = {
        set_availability: {
            name: "waledayotwe.arch",
            available: true,  
        }
    };

        const signContract = await signingClient.execute(
            accounts[0].address,
            contractAddress,
            msg,
            "auto",
        );
        

        console.log("signContract: ", signContract);

    } catch (error) {
        console.log('error', error)
    }
}


// update metadata
async function updateMetadata() {
    try {
    
    const msg = {
        update_metadata: {
            name: "waledayotwe.arch",
            update: {
                description: "IPFS link for user profile for waledayoele",
                // Other properties stay the same
                image: "ipfs://QmNoMUgTM82EGaTCTnuEUJDusV21UEGSgKM5RhM1C9N3WE",
                accounts: [{username: "archid-protocol",profile: "https://github.com/archid-protocol",account_type: "github",verfication_hash: null}],
                websites: [{url: "https://archid.app",domain: "dapp.archid.arch",verfication_hash: null}]
            },  
        }

    };

        const signContract = await signingClient.execute(
            accounts[0].address,
            contractAddress,
            msg,
            "auto",
        );
        

        console.log("signContract: ", signContract);

    } catch (error) {
        console.log('error', error)
    }
}


// job Requst
async function jobRequest() {
    try {
    
        const msg = {
            job_request: {
                contractor_domain: "waledayotwe.arch",
                duration: 1,
            }
        };

        const amount = '1000000000000000000'

        let funds = [{
            denom: 'aconst',
            amount: amount,
        }]
    

        const signContract = await signingClient.execute(
            accounts[0].address,
            contractAddress,
            msg,
            "auto",
            "send job request",
            funds
        );
        

        console.log("signContract: ", signContract);

    } catch (error) {
        console.log('error', error)
    }
}

// accept job Requst
async function acceptJobRequest() {
    try {
    
        const msg = {
            accept_request: {
                job_id: 1,
            }
        };

        const amount = '1000000000000000000'
    
        const signContract = await signingClient.execute(
            accounts[0].address,
            contractAddress,
            msg,
            "auto",
        );
        

        console.log("signContract: ", signContract);

    } catch (error) {
        console.log('error', error)
    }
}

// withrawal Requst
async function withdrawalRequest() {
    try {
    
        const msg = {
            withdrawal_request: {
                job_id: 1,
            }
        };
    
        const signContract = await signingClient.execute(
            accounts[0].address,
            contractAddress,
            msg,
            "auto",
        );
        

        console.log("signContract: ", signContract);

    } catch (error) {
        console.log('error', error)
    }
}


// approve withrawal Requst
async function approveWithdrawalRequest() {
    try {
    
        const msg = {
            approve_withdrawal: {
                job_id: 1,
            }
        };
    
        const signContract = await signingClient.execute(
            accounts[0].address,
            contractAddress,
            msg,
            "auto",
        );
        

        console.log("signContract: ", signContract);

    } catch (error) {
        console.log('error', error)
    }
}

//  withraw money
async function withdraw() {
    try {
    
        const msg = {
            withdraw: {
                job_id: 1,
            }
        };
    
        const signContract = await signingClient.execute(
            accounts[0].address,
            contractAddress,
            msg,
            "auto",
        );
        

        console.log("signContract: ", signContract);

    } catch (error) {
        console.log('error', error)
    }
}

//  reject job request
async function rejectRequest() {
    try {
    
        const msg = {
            reject_request: {
                job_id: 2,
            }
        };
    
        const signContract = await signingClient.execute(
            accounts[0].address,
            contractAddress,
            msg,
            "auto",
        );
        

        console.log("signContract: ", signContract);

    } catch (error) {
        console.log('error', error)
    }
}

//  review
async function review() {
    try {
    
        const msg = {
            review: {
                job_id: 1,
                review: "IPFS link for review"
            }
        };
    
        const signContract = await signingClient.execute(
            accounts[0].address,
            contractAddress,
            msg,
            "auto",
        );
        

        console.log("signContract: ", signContract);

    } catch (error) {
        console.log('error', error)
    }
}

// createProfile()
// hourlyRate()
// availability()
// updateMetadata()
jobRequest()
// acceptJobRequest()
// withdrawalRequest()
// approveWithdrawalRequest()
// withdraw()
// rejectRequest()
// review()