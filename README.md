# Compound protocol

```rust
struct Compound {
    token_address: ActorId,  // address of token contract
    ctoken_address: ActorId, // address of ctoken contract
    owner_address: ActorId,  // address of owner
    lend_data: LendData,     // info realted to lend tokens
    borrow_data: BorrowData, // info realted to borrow tokens
    user_deposits: BTreeMap<ActorId, BTreeSet<Payment>>, // map of address -> lend payment
    user_borrows: BTreeMap<ActorId, BTreeSet<Payment>>,  // map of address -> borrow payment
}

pub struct LendData {
    pub ctoken_rate: u128,    // token cost * `ctoken_rate` = Ctoken cost
    pub interest_rate: u128,  // user will earn `interest_rate` percent asset
    pub max_deposit_id: u128, // id of the last lend
}

pub struct BorrowData {
    pub collateral_factor: u128, // user can borrow callateral * `collateral_factor`
    pub borrow_rate: u128,       // user will have to pay borrowed amount * `borrow_rate`
    pub max_borrow_id: u128,     // id of the last borrow
}

pub struct Payment {
    pub amount: u128,         // how much tokens in the lend/borrow
    pub ctokens_amount: u128, // 0 if payment is borrow, otherwise how much ctokens got for lending
    pub interest_rate: u128,  // `interest_rate` or `borrow_rate`
    pub payment_time: u64,    // timestamp when payment was made
    pub id: u128,             // payment id 
}

impl Payment {
    // returns how amount increased 
    pub fn count_interest(&self, amount: u128, time_now: u64) -> u128; 
}

impl Compound {
    // `msg::source` lend `amount` of tokens
    pub async fn lend_tokens(&mut self, amount: u128); 
    
    // `msg::source` borrow `amount` of tokens
    pub async fn borrow_tokens(&mut self, amount: u128); 
    
    // `msg::source` redeems tokens with ids in `lend_to_redeem` and refunds borrows in `amount_per_borrow`
    pub async fn redeem_tokens(
        &mut self,
        lend_to_redeem: BTreeSet<u128>,
        amount_per_borrow: BTreeMap<u128, u128>,
    );
    
    // returns how much tokens are possible to borrow for `user`
    fn borrow_possible(&self, user: ActorId) -> u128; 
    
    // returns is redeem posible with those params
    fn redeem_posible(
        &self,
        user: ActorId,
        lend_to_redeem: &BTreeSet<u128>,
        amount_per_borrow: &BTreeMap<u128, u128>,
    ) -> bool;
    
    // add payment to `container`
    fn insert_payment(
        address: ActorId,
        container: &mut BTreeMap<ActorId, BTreeSet<Payment>>,
        payment: &mut Payment,
    );
    
    // delete all payments where amount = 0
    fn delete_payments(&mut self, user_address: ActorId);
    
    // count how much ctokens corresponds to `tokens_amount`
    fn count_ctokens(&mut self, tokens_amount: u128) -> u128;
    
    // count how much tokens corresponds to `ctokens_amount`
    fn count_tokens(&mut self, ctokens_amount: u128) -> u128;
    
    // updates `tokens` -> `ctokens` rate
    fn update_rate(&mut self);
}
```
