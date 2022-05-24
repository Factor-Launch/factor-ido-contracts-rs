use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{env, log, near_bindgen, AccountId, PanicOnDefault, Promise, PromiseOrValue};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct PubTokenSale {
    is_open: bool,
    native_token_price: U128,
    native_token_account_id: AccountId,
    native_token_balance: U128,
    swap_token_account_id: AccountId,
    swap_token_price: U128,
    swap_token_balance: U128,
    wallet_cap_limit: U128,
    proceeds_address: AccountId,
}

#[near_bindgen]
impl PubTokenSale {
    #[payable]
    pub fn perform_swap(&mut self) {
        self.swap_token_balance =
            U128(u128::from(self.swap_token_balance.0) + env::attached_deposit());
        self.native_token_balance =
            U128(u128::from(self.native_token_balance.0) + env::attached_deposit());
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
        self.native_token_balance = amount;
        log!("the amount of tokens in this address is {}", amount.0);
        PromiseOrValue::Value(U128::from(0))
    }
}

// second we're going to add a function that accepts near and then dispenses 1 of the tokens 1:1 for simplicity ===>

// then you add oracle functionality that then calcultes the amount being dispensed and then that's it for the MVP then we move on to testing it
