# Crowdsale (ICO)

## Introduction
An initial coin offering (ICO) or initial currency offering is a type of funding using cryptocurrencies. It is a form of crowdfunding, in which a quantity of cryptocurrency is sold in the form of "tokens" ("coins") to speculators or investors, in exchange for legal tender or other (generally established and more stable) cryptocurrencies such as Bitcoin or Ether. The tokens are promoted as future functional units of currency when the ICO's funding goal is met and the project successfully launches.

## Interface
### Source files
1. `messages.rs` - contains function of the fungible token contract. Crowdsale contract interacts with fungible token contract through transfer_tokens function:
```rust
pub async fn transfer_tokens(
    token_id: &ActorId, // - the fungible token contract address
    from: &ActorId, // - the sender address
    to: &ActorId, // - the recipient address
    amount: u128, // - the amount of tokens
) 
```
This function sends a message (the action is defined in the enum IcoAction) and gets a reply (the reply is defined in the enum IcoEvent):
```rust
    let _transfer_response: FTEvent = msg::send_and_wait_for_reply(
        *token_id,
        FTAction::Transfer {
            from: *from,
            to: *to,
            amount,
        },
        0,
    )
    .unwrap()
    .await
    .expect("Error in transfer");
```

2. `asserts.rs` - contains asserts functions: `owner_message` and `not_zero_address`. 
- `owner_message` checks if `msg::source()` is equal to `owner`. Otherwise, it panics:
```rust
pub fn owner_message(owner: &ActorId, message: &str) {
    if msg::source() != *owner {
        panic!("{}: Not owner message", message)
    }
}
```
- `not_zero_address` checks if `address` is not equal to `ZERO_ID`. Otherwise, it panics:
```rust
pub fn not_zero_address(address: &ActorId, message: &str) {
    if address == &ZERO_ID {
        panic!("{}: Zero address", message)
    }
}
```

3. `lib.rs` - defines the contract logic.

### Structs
The contract has the following structs:
```rust
struct IcoContract {
    ico_state: IcoState,
    start_price: u128,
    price_increase_step: u128,
    time_increase_step: u128,
    tokens_sold: u128,
    tokens_goal: u128,
    owner: ActorId,
    token_address: ActorId,
    token_holders: BTreeMap<ActorId, u128>,
}
```
where:
- `ico_state` is `IcoState` struct which consists of:
```rust
pub struct IcoState {
    pub ico_started: bool, // true if ICO was started
    pub start_time: u64, // time when ICO was started, otherwise is zero
    pub duration: u64, // duration of the ICO, otherwise is zero
    pub ico_ended: bool, // true if ICO was ended
}
```
- `start_price` // initial price of tokens
- `price_increase_step` // how much does the price increase
- `time_increase_step` // the period of time after which the price increases
- `tokens_sold` // how many tokens were sold
- `tokens_goal` // how many tokens are we going to sell
- `owner` // contract owner
- `token_address` // fungible token address 
- `token_holders` // the list of buyers and the number of tokens they bought

### Functions


## Conclusion

