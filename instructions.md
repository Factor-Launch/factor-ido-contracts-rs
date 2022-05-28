# Order of Commands 
~/launch-local-near-cluster.sh
export ID="sonny.test.near"
export TOKEN_ID="token.sonny.test.near"
export TOKENSALE_ID="token-sale.test.near"
local_near send test.near sonny.test.near 1000000
local_near create-account token.sonny.test.near --initialBalance 1000 --masterAccount test.near
local_near deploy --wasmFile res/fungible_token.wasm --accountId $TOKEN_ID
near call $TOKEN_ID new '{"owner_id": "'$ID'", "total_supply": "1000000000000000", "metadata": { "spec": "ft-1.0.0", "name": "Sonny Coin", "symbol": "SCN", "decimals": 8 }}' --accountId $TOKEN_ID
near call $TOKEN_ID storage_deposit '' --accountId $ID --amount 0.00125
local_near call $TOKEN_ID ft_transfer '{"receiver_id": "sonny.test.near", "amount": "1000000000000000" }' --accountId sonny.test.near
## Deploy the tokensale contract
1. create an account for that contract
2. deploy the wasm to that address
3. Call the new function w/ arguments
4. Deposit Tokens to the contract
5. Deposit NEAR to the contract and see what the behavior is 

name very mix fossil reflect quarter change dry method tag bachelor web