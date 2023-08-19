# Connector
Create the connection to data service like infura/etherscan to get the events and on-chain data from smart contracts, then parse it and send to tss for signing.

## How to update config to fetch desired data from connector:
Currently, the connector can fetch the two major type of data as per the user configuration.
* fetch the event data of the smart contract(like transfer events).
* fetch the swap price of the token to other token as per user config.

### Fetch swap price of the token:
The connector can fetch the swap price of a token to other as per the user configuration added in mongoDB Server.

To update the config on MongoDB server first we need to run tango using below commands:
`cargo build --release`

`target/release/tango-node --db-url DB_URL`

Then we can use curl request to add the configuration to the Mongo Server:

`curl --location --request POST 'http://127.0.0.1:8080/tokenswap' \
--header 'Content-Type: application/json' \
--data-raw '[
{
"chain": "ethereum",
"chain_endpoint": "https://a507940678ae4740a727967aa8566e08.eth.rpc.rivet.cloud/",
"exchange": "Uniswap",
"exchange_address": "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D",
"exchange_endpoint": "https://api.etherscan.io/api?module=contract&action=getabi&address=0x7a250d5630B4cF53[…]2C5dAcb4c659F2488D&apikey=1M6T2FCU18IEG2K7D8EWFM5Z8CH6QEUESM",
"token": "WETH",
"token_address": "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
"token_endpoint": "https://api.etherscan.io/api?module=contract&action=getabi&address=0xC02aaA39b223FE8D[…]4F27eAD9083C756Cc2&apikey=1M6T2FCU18IEG2K7D8EWFM5Z8CH6QEUESM",
"swap_token": "USDT",
"swap_token_address": "0xdAC17F958D2ee523a2206206994597C13D831ec7",
"swap_token_endpoint": "https://api.etherscan.io/api?module=contract&action=getabi&address=0xdAC17F958D2ee523[…]06994597C13D831ec7&apikey=1M6T2FCU18IEG2K7D8EWFM5Z8CH6QEUESM"
}
]'`

These configurations will be fetch by the connector and then connector will fetch the swap price of the token.
After fetching the current swap price, the connector will log the data and insert the data into the db.

Fetched sample data
`Swap WETH token to MATIC token:  1700.045996379865`


### Fetch accounts data from the polkadot chain:
The tango is now compatible with polkadot chains also. We can fetch the on-chain data from the polkadot chains.



##  Development plan:
- [] node can get the events data from erc20 contract
- [] parse it to internal event data structure
- [] event send to tss via mpsc
