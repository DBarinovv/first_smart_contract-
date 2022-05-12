# ICO contract

## Token fields
- `start_time`: `Timestamp` or `u64`
- `end_time` or `duration`: `Timestamp` or `u64`
- `price`: `u128`
- `price_step`: `u128`
- `time_step`: `u64` in seconds
- `tokens_sold`: `u128` (maybe `HashMap<ActorId, u128>` if tokens are transfered after end_time)
- `owner`: `ActorId`
- `token_id`: `ActorId`
- `is_ended`: `bool`


## Functions
- `fn update_end()` (if timestamp now > `self.end_time` modifies `self.is_ended`)
- `fn update_price()` (checks timestamp and updates price if needed)
- `pub fn buy_tokens(tokens_cnt: u128)` (sender buys tokens_cnt tokens)
- `pub fn end_sale()` (ends token sale)
