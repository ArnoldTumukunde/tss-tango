import { ethers } from "ethers";
const provider = new ethers.providers.JsonRpcProvider(
  "http://127.0.0.1:8545"
);
const fs = require('fs');

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
const priv_key1 = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
const account2 = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";
const priv_key2 = "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
const account3 = "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC";
const priv_key3 = "0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a";
const account4 = "0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65";
const priv_key4 = "0x47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a";

const account5 = "0x90F79bf6EB2c4f870365E785982E1f101E93b906"

const wallet1 = new ethers.Wallet(priv_key1, provider);
const wallet2 = new ethers.Wallet(priv_key2, provider);
const wallet3 = new ethers.Wallet(priv_key3, provider);
const wallet4 = new ethers.Wallet(priv_key4, provider);

const distribute_balance = async() => {
  const acc1 = contract.connect(wallet1);
  const balanceOfacc1 = await contract.balanceOf(account1);
  const division_balance = balanceOfacc1.div(4);
  const tx1 = await acc1.transfer(account2, division_balance);
  const tx2 = await acc1.transfer(account3, division_balance);
  const tx3 = await acc1.transfer(account4, division_balance);
  await Promise.all([tx1.wait(),tx2.wait(),tx3.wait()]);
}

const sendToken1 = async () => {
  for (let i = 0; i < 250; i++) {
    const acc1 = contract.connect(wallet1);
    const tx = await acc1.transfer(account5, "1000000000000000000");
    await tx.wait();
  }
};

const sendToken2 = async () => {
  for (let i = 0; i < 250; i++) {
    const acc1 = contract.connect(wallet2);
    const tx = await acc1.transfer(account5, "1000000000000000000");
    await tx.wait();
  }
};

const sendToken3 = async () => {
  for (let i = 0; i < 250; i++) {
    const acc1 = contract.connect(wallet3);
    const tx = await acc1.transfer(account5, "1000000000000000000");
    await tx.wait();
  }
};

const sendToken4 = async () => {
  for (let i = 0; i < 250; i++) {
    const acc1 = contract.connect(wallet4);
    const tx = await acc1.transfer(account5, "1000000000000000000");
    await tx.wait();
  }
};

const main = async () => {

  const name = await contract.name();
  const symbol = await contract.symbol();
  const totalSupply = await contract.totalSupply();
  await distribute_balance();

  const balanceOfacc1 = await contract.balanceOf(account1);
  const balanceOfacc2 = await contract.balanceOf(account2);
  const balanceOfacc3 = await contract.balanceOf(account3);
  const balanceOfacc4 = await contract.balanceOf(account4);

  console.log("balanceOfacc1", balanceOfacc1.toString());
  console.log("balanceOfacc2", balanceOfacc2.toString());
  console.log("balanceOfacc3", balanceOfacc3.toString());
  console.log("balanceOfacc4", balanceOfacc4.toString());

  console.log(`\nReading from ${contract_address}\n`);
  console.log(`Name: ${name}`);
  console.log(`Symbol: ${symbol}`);
  console.log(`Total Supply: ${totalSupply}\n`);
  var a = performance.now();
  await Promise.all([sendToken1(), sendToken2(), sendToken3(), sendToken4()]);
  var b = performance.now();
  const total_time = (b - a) / 1000;
  console.log("Total time taken by event emitter  ", total_time, "secs");
};

main();
