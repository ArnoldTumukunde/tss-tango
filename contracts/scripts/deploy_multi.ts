import { ethers } from 'hardhat' 


let token:any;
let contractAddress: any[] = [];
let numberOfContractToDeploy = 2; // number Of Contract To Deploy

async function main() {
    const amount = 1000000000000; //Replace amount with amount you want mint
  
    const Token = await ethers.getContractFactory("ERC20");

    for(let itr = 0; itr < numberOfContractToDeploy; itr++){
        token = await Token.deploy(`tango${itr}`, `TNG${itr}`);
        await token.deployed();
        contractAddress.push(token.address);
        console.log(token.address);
    }

    return contractAddress;

}

main()

module.exports = {main}


