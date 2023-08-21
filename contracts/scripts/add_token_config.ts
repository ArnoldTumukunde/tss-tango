import { ethers } from "hardhat";

interface SwapToken {
  chain: String,
  chain_endpoint: String,
  exchange: String,
  exchange_address: String,
  exchange_endpoint: String,
  token: String,
  token_address: String,
  token_endpoint: String,
  swap_token: String,
  swap_token_address: String,
  swap_token_endpoint: String,
}

async function main() {
    const[owner,addr1] = await ethers.getSigners();
    const provider = await ethers.getDefaultProvider();


  const swap_input:SwapToken  = {
    chain:"localhost",
    chain_endpoint: "localhost",
    exchange: "local_chain",
    exchange_address: "localhost",
    exchange_endpoint: "localhost",
    token: "String",
    token_address: "String",
    token_endpoint: "String",
    swap_token: "String",
    swap_token_address: "String",
    swap_token_endpoint: "String",
  };
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
