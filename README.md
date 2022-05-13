# ICO contract

## Token fields
- `tokens_goal`: `u128`,
- `price`: `u128`,
- `price_step`: `u128`,
- `tokens_step`: `u128`,
- `tokens_sold`: `u128`,
- `owner`: `ActorId`, 
- `token_id`: `ActorId`,
- `was_ended`: `bool`,
- `was_stated`: `bool`,
- `token_holders`: `BTreeMap<ActorId, u128>`,


## Functions
` `pub fn start_contract()` (starts salling tokens)
- `fn update_end()` (if timestamp now > `self.end_time` modifies `self.is_ended`)
- `fn update_price()` (checks timestamp and updates price if needed)
- `pub fn buy_tokens(tokens_cnt: u128)` (sender buys tokens_cnt tokens)
- `pub fn end_sale()` (ends token sale)
