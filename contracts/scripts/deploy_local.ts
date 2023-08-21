const hre = require('hardhat');
const cron = require("cron");
const { Contract } = require('hardhat/internal/hardhat-network/stack-traces/model');
// const { ethers } = require('hardhat');
import { ethers } from 'hardhat';

let Token :any;
let token : any;

async function main() {
    const amount = 1000000000000; //Replace amount with amount you want mint
    Token = await ethers.getContractFactory("ERC20");
    token = await Token.deploy("tango", "TNG");
  
    await token.deployed();
  
    console.log("Contract deployed to ", token.address);

}
const transfer_dummy = async ()=>{

    const addresses = await ethers.getSigners();
    const add1 = addresses[1];
    const val = 1*10**10;
    await token.transfer(add1.address,BigInt(val));
    const add1Bal = await token.balanceOf(add1.address);
    console.log(add1Bal.toString());

}


const listenToEvent = async () => {
    
    token.on("Transfer",(from: any,to: any,value: any,event: any) => {
        let info = { 
            from: from,
            to: to,
            value: ethers.utils.formatUnits(value,6),
            data: event,
        };
        console.log(JSON.stringify(info,null,4));
    })

} 


main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
  });

const cronJob = cron.job('*/5 * * * * *', async () => {
    console.log('Checking...');
    await transfer_dummy();
    await listenToEvent();
    console.log('Complete!\n');
});

cronJob.start();
