import axios from "axios";
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

  const Swap_price = await ethers.getContractFactory("TokenSwap");
  const swap_price = await Swap_price.deploy();

  await swap_price.deployed();

  console.log("address = ",swap_price, swap_price.address);

  const swap_input:SwapToken  = {
    chain:"localChain",
    chain_endpoint: "http://127.0.0.1:8545",
    exchange: "localChain",
    exchange_address: swap_price.address,
    exchange_endpoint: "contracts/artifacts/contracts/swap_price.sol/TokenSwap.json",
    token: "WETH",
    token_address: "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0", // Please update the deployed toke address.
    token_endpoint: "https://api.etherscan.io/api?module=contract&action=getabi&address=0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2&apikey=1M6T2FCU18IEG2K7D8EWFM5Z8CH6QEUESM",
    swap_token: "USDT",
    swap_token_address: "0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9", // Please update the deployed toke address.
    swap_token_endpoint: "https://api.etherscan.io/api?module=contract&action=getabi&address=0xdAC17F958D2ee523a2206206994597C13D831ec7&apikey=1M6T2FCU18IEG2K7D8EWFM5Z8CH6QEUESM",
  };
  axios.post("http://127.0.0.1:8080/tokenswap", [swap_input])
  .then(function (response) {
    console.log(response);
  })
  .catch(function (error) {
    console.log(error);
  });

 const InsertToken = () => {
     const token_input = {
       token: "WETH",
       token_address: "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0"
     }
     axios.post("http://127.0.0.1:8080/tokens", token_input)
       .then(function (response) {
         alert(response.status);
         console.log(response);
       })
       .catch(function (error) {
         console.log(error);
       });
   }
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
