export ID="sonny.test.near"
export TOKEN_ID="token.sonny.test.near"
export TOKENSALE_ID="tokensale.sonny.test.near"
# near create-account token.sonny.test.near  --initialBalance 200 --masterAccount sonny.test.near
near deploy --wasmFile res/fungible_token.wasm --accountId $TOKEN_ID
near call $TOKEN_ID new '{"owner_id": "'$ID'", "total_supply": "1000000000000000", "metadata": { "spec": "ft-1.0.0", "name": "Sonny Coin", "symbol": "SCN", "decimals": 8 }}' --accountId $TOKEN_ID
near call $TOKEN_ID storage_deposit '' --accountId $ID --amount 0.00125
near call $TOKEN_ID storage_deposit '' --accountId $TOKEN_ID --amount 0.00125
near call $TOKEN_ID ft_transfer '{"receiver_id": "'$ID'", "amount": "1000000000000000"}' --accountId $TOKEN_ID --amount 0.000000000000000000000001