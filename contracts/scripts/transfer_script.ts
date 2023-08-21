import {ethers} from "ethers";
import cron from "cron";
const provider = new ethers.providers.JsonRpcProvider(
  "https://rinkeby.infura.io/v3/0e188b05227b4af7a7a4a93a6282b0c8"
);
const contract_address = "0x210656D23e8B12822D34f80A33b81264Ed0b6aC6";
const ERC20_ABI = [
  "function name() view returns (string)",
  "function symbol() view returns (string)",
  "function totalSupply() view returns (uint256)",
  "function balanceOf(address) view returns (uint)",
  "function transfer(address to, uint amount) returns (bool)",
  "event Transfer(address indexed from, address indexed to, uint amount)",
];
const contract = new ethers.Contract(contract_address, ERC20_ABI, provider);
const account1 = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
const account2 = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";

const PrivateKey1 = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

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
  const name = await contract.name();
  const symbol = await contract.symbol();
  const totalSupply = await contract.totalSupply();

  console.log(`\nReading from ${contract_address}\n`);
  console.log(`Name: ${name}`);
  console.log(`Symbol: ${symbol}`);
  console.log(`Total Supply: ${totalSupply}\n`);
};


const listenToTransfer = async()=>{
    const block = await provider.getBlockNumber();
    console.log(block)
    const transferEvents = await contract.queryFilter('Transfer', block - 1, block)
    console.log(JSON.stringify(transferEvents))
}


const cronJob = cron.job("*/5 * * * * *", async () => {
  console.log("Checking...");
    main();
    listenToTransfer()  
  console.log("Complete!\n");
});

cronJob.start();
sendToken();

