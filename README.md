Tech description

# Compound 

## Structs
```rust
CErc20 {
    fn mint(amount: U256) -> U256;

    fn borrow(amount: U256) -> U256;

    fn borrowRatePerBlock() -> U256; // state function

    fn borrowBalanceCurrent(address: ) -> U256;

    fn repayBorrow(amount: U256) -> U256;
}
```

## Functions
