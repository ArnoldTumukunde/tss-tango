import { ethers } from "ethers";
const provider = new ethers.providers.JsonRpcProvider(
    "http://127.0.0.1:8545"
);

// it is the first contract address
const contract_address = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
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
};

const main = async () => {

    const name = await contract.name();
    const symbol = await contract.symbol();
    const totalSupply = await contract.totalSupply();
    console.log(`\nReading from ${contract_address}\n`);
    console.log(`Name: ${name}`);
    console.log(`Symbol: ${symbol}`);
    console.log(`Total Supply: ${totalSupply}\n`);
    for (let i = 0; i < 1000; i++) {
        console.log("Sending token for {}", i);
        await sendToken();
    }
};


main();