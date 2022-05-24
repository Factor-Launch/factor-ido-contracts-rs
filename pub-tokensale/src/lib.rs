use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{
    env, log, near_bindgen, result_serializer, AccountId, Balance, PanicOnDefault, Promise,
    PromiseOrValue,
};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct PubTokenSale {
    // is_open: bool,
    // native_token_price: U128,
    native_token_account_id: AccountId,
    native_token_balance: u128,
    // swap_token_account_id: AccountId,
    // swap_token_price: U128,
    swap_token_balance: u128,
    // wallet_cap_limit: U128,
    // proceeds_address: AccountId,
}

#[near_bindgen]
impl PubTokenSale {
    #[result_serializer(borsh)]
    pub fn new() -> Self {
        Self {
            native_token_account_id: "token.test.near".to_string(),
            native_token_balance: 0,
            swap_token_balance: 0,
        }
    }

    #[payable]
    pub fn perform_swap(&mut self) {
        self.swap_token_balance = u128::from(self.swap_token_balance) + env::attached_deposit();
        self.native_token_balance = u128::from(self.native_token_balance) + env::attached_deposit();
        Promise::new(self.native_token_account_id.to_string()).transfer(env::attached_deposit());
    }
}

#[near_bindgen]
impl FungibleTokenReceiver for PubTokenSale {
    fn ft_on_transfer(
        &mut self,
        _sender_id: ValidAccountId,
        amount: U128,
        _msg: String,
    ) -> PromiseOrValue<U128> {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.native_token_account_id,
            "Only supports deposits from the 1 Token Address"
        );
        self.native_token_balance = u128::from(amount);
        log!("the amount of tokens in this address is {}", amount.0);
        PromiseOrValue::Value(U128::from(0))
    }
}

/*
I am almost positive that this doesn't work as intended let's deploy some of these
to kurtosis and see how they behave and we can move on from there and see
where we're fucking  this up
*/
