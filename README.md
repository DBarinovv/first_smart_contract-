Tech description

# Compound 

Consists of two parts:
- Main borrow logic.
- Interaction with many users and tokens. Algorithm for finding risk weights, interest rate etc.

## Borrowing

### Functions

We need those functions for each kind of token (tokens are ERC20):

```rust
fn lend(amount: U256); // Put amount of current tokens under interest

fn borrow(amount: U256); // Borrow amount of current tokens

fn repay_borrow(amount: U256); // Repay borrow on current token

fn get_debt_amount(address: ActorID) -> U256 // State function. Returns how much tokens user need to bring back

fn get_lend_amount(address: ActorID) -> U256 // State function. Returns how much tokens user has on his deposit
```

## DeFi

Some functions (in progress):
```rust
- fn enter_market(address: ActorID); 
- fn exit_market(address: ActorID);
```
