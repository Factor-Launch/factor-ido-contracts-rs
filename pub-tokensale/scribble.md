✅ = done
🟧 = in progress
❌ = not done

# What Data is important to nail in this contract

    1. State of the TokenSale [Open, Closed, Closing]
    2. How much of the Native Token is there in the contract
    3. What the Price of the Token is
    4. How Much 1 Wallet is allowed to buy
    5. What the swap token is
    6. Price of the swap token


## Now that we have some of these large high level concepts let's rank them in terms of what it's our capability, what can stretch us and what can paralyze us

### What can get done right now
    1. State of Tokensale ✅ 
    2. Have Tokensale State only controllable by the owner ❌
    3. 


# We have an unanswered question here, how do cross contract & token interactions work here b/c I don't get it 
    
What would be a good place to see these things modelled in rust terms?


A swap contract?

A Lending Contract? 

The example deFi contract?


===========
# Let's outline the progression of these things
    0. Deploy & Transfer non-near tokens ✅
    1. A Simple Deposit Contract with NEAR 🟧
    2. A Simple Deposit & Withdraw contract with a Token
    3. A Simple Swap [1 way] contract with NEAR with a Token
    4. A Simple Swap [1 way] contract with 1 Token [from CL] & another
    5. A Two-Way Swap Contract with 1 Token and Another

So let's take on each of these and we'll be well on our way to killing this :)


Take a look at the defi contract in the FT & take notes 

Ok let's do some overview of how the sample DeFi contract does it's magic

so start here are the imports

1. Fungible Token Reciever from the FT Standard ==> I think this is the interface we'll be using to interact with external NEP-141s
2. Then we have Borsh for Biolerplate
3. U128 for JSON Serialization Reasons

Then we have 