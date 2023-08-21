import { ethers } from "hardhat";
import Abi from "../artifacts/contracts/swap_price.sol/TokenSwap.json";

const abi = Abi.abi;  
let amount = 10; //provided amount 
const price = async()=>{
    
    const[owner,addr1] = await ethers.getSigners();
    const provider = await ethers.getDefaultProvider();
    const address = "0x5FbDB2315678afecb367f032d93F642f64180aa3"; //deployed address of local

    const swap = new ethers.Contract(address,abi,provider);

    const after = await swap.connect(owner).getAmountsOut(owner.address,addr1.address,amount);
   
    const value = after[0].toNumber();
    console.log(after[0], value);
}

price();