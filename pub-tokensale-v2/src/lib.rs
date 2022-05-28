use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, Gas, PanicOnDefault, Promise};

// const NO_DEPOSIT: Balance = 0;
// const BASIC_GAS: Gas = Gas(5_000_000_000_000);

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct PubTokenSale {
    native_token_account_id: AccountId,
}

#[near_bindgen]
impl PubTokenSale {
    #[result_serializer(borsh)]
    pub fn new(native_token_account_id: AccountId) -> Self {
        Self {
            native_token_account_id,
        }
    }
    // accept tokens
    pub fn accept_tokens(&mut self, token_amt: u128, token_id: AccountId) -> u128 {
        // require that token_id and native_token_account_id are the same
        assert_eq!(
            self.native_token_account_id, token_id,
            "Has to match the native token"
        );
        let amount = env::attached_deposit();
        Promise::new(token_id).function_call(
            "ft_transfer".to_string(),
            json!({ "receiver_id" : env::predecessor_account_id(), "amount" : U128(token_amt), "msg": "" })
                .to_string()
                .as_bytes()
                .to_vec(),
            38600000000000000000000,
            env::prepaid_gas(),
        );
        log!("success");
        amount
    }
    // take in NEAR and then dispense a token
    pub fn perform_swap(&mut self) {
        // perform oracle call
        // do math -> calculate amount of tokens to be dispensed
        Promise::new(self.native_token_account_id.clone()).function_call(
            "ft_transfer".to_string(),
            json!({"receiver_id": env::predecessor_account_id(), "amount": U128(12345), "msg": ""})
                .to_string()
                .as_bytes()
                .to_vec(),
            38600000000000000000000,
            env::prepaid_gas(),
        );
    }
}
