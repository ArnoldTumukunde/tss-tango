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

const distribute_amounts = async () => {
  const accounts = await provider.listAccounts();
  const main_balance = await contract.balanceOf(accounts[0]);
  const amount = (main_balance / 19).toString();
  console.log(amount);
  const main_signer = provider.getSigner(accounts[0]);
  const main_wallet = contract.connect(main_signer);
  var tx_list = [];
  for (let i = 1; i < accounts.length - 1; i++) {
    tx_list.push(await main_wallet.transfer(accounts[i], amount));
  }
  await Promise.all(tx_list);

  //print everyone's balance
  for (let i = 0; i < accounts.length; i++) {
    const balance = await contract.balanceOf(accounts[i]);
    console.log(`Balance of ${accounts[i]}: ${balance}`);
  }
}


const main = async () => {

  const name = await contract.name();
  const symbol = await contract.symbol();
  const totalSupply = await contract.totalSupply();

  console.log(`\nReading from ${contract_address}\n`);
  console.log(`Name: ${name}`);
  console.log(`Symbol: ${symbol}`);
  console.log(`Total Supply: ${totalSupply}\n`);

  const accounts = await provider.listAccounts();
  var wallets: any = [];
  for (let i = 0; i < accounts.length; i++) {
    wallets.push(await provider.getSigner(accounts[i]));
  }

  console.log("accounts: ", accounts);

  await distribute_amounts();

  var transfer_ls_functions = [];

  const receiver_addr = accounts.pop();
  wallets.pop();

  for (let i = 0; i < wallets.length; i++) {
    const send_token_func = async () => {
      for (let j = 0; j < 53; j++) {
        const acc = contract.connect(wallets[i]);
        const tx = await acc.transfer(receiver_addr, "100000");
        await tx.wait();
      }
    }
    transfer_ls_functions.push(send_token_func());
  }
  console.log("length of transfers", transfer_ls_functions.length);
  var a = performance.now();
  await Promise.all(transfer_ls_functions);
  var b = performance.now();
  const total_time = (b - a) / 1000;
  console.log("Total time taken by event emitter  ", total_time, "secs");
  fs.writeFile(`${total_time}.js`, '', (error: any) => {
  });
};
main();

