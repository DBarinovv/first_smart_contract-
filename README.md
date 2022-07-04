Tech description

# Compound 

Consists of two parts:
- Main borrow logic.
- Interaction with many users and tokens. Algorithm for finding risk weights, interest rate etc.

## Borrowing

### Functions

We need those functions for each kind of token (tokens are ERC20):

```rust
fn borrow(amount: U256) // Borrow amount of current tokens

fn repay_borrow(amount: U256) // Repay borrow on current token

fn get_debt_amount(address: ActorID) -> U256 // State function. Returns how much tokens user need to bring back
```
