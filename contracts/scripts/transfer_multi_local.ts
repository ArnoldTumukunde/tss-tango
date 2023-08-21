import  {ethers} from "ethers";
import * as deploy1 from "./deploy_multi";
const provider = new ethers.providers.JsonRpcProvider("http://127.0.0.1:8545");
const ERC20_ABI = [
  "function name() view returns (string)",
  "function symbol() view returns (string)",
  "function totalSupply() view returns (uint256)",
  "function balanceOf(address) view returns (uint)",
  "function transfer(address to, uint amount) returns (bool)",
  "event Transfer(address indexed from, address indexed to, uint amount)",
];
let contract:any;
const account1 = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
const account2 = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";

const PrivateKey1 ="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

const wallet = new ethers.Wallet(PrivateKey1, provider);
const sendToken = async () => {
  const acc1 = contract.connect(wallet);
  const tx = await acc1.transfer(account2, "1000000000000000000");
  await tx.wait();
  const balanceOfSender = await contract.balanceOf(account1);
  const balanceOfReciever = await contract.balanceOf(account2);
  console.log(`\nBalance of sender: ${balanceOfSender}`);
  console.log(`Balance of reciever: ${balanceOfReciever}\n`);
};

const main = async () => {
  let contract_address = await deploy1.main();
  if(Array.isArray(contract_address) && contract_address != undefined){
    console.log("contract_address2:",contract_address);
      while(true){
      for(let itr = 0; itr < contract_address.length;itr++) {
        contract = new ethers.Contract(contract_address[itr], ERC20_ABI, provider);
        sendToken();
        const name = await contract.name();
        const symbol = await contract.symbol();
        const totalSupply = await contract.totalSupply();
        console.log('\nReading from',contract_address[itr]);
        console.log('Name: ',name);
        console.log('Symbol: ',symbol);
        console.log('Total Supply: ',totalSupply);
        await new Promise((r) => setTimeout(r, 1000));
      }
    }
  } else{
    console.log("No Contract deployed");
  }
};
main();
