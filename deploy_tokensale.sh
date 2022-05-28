export ID="sonny.test.near"
export TOKEN_ID="token.sonny.test.near"
export TOKENSALE_ID="tokensale.sonny.test.near"
near create-account tokensale.sonny.test.near --initialBalance 400 --masterAccount sonny.test.near
near deploy --wasmFile res/pub_tokensale.wasm --accountId $TOKENSALE_ID
# near call $TOKENSALE_ID new '{"native_token_account_id": "token.sonny.test.near"}' --accountId $ID