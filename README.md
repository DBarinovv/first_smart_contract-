# ICO contract

## Token fields
- `ico_state`: `IcoState`,
  + `ico_started`: `bool`,
  + `start_time`: `u64`,
  + `duration`: `u64`,
- `current_price`: `u128`,
- `price_increase_step`: `u128`
- `time_increase_step`: `u128`,
- `tokens_sold`: `u128`,
- `tokens_goal`: `u128`,
- `owner`: `ActorId`, 
- `token_id`: `ActorId`,
- `token_holders`: `BTreeMap<ActorId, u128>`,

## Functions
- `fn get_tokens()` (mint and transfer tokens to the contract)
` `pub fn start_ico()` (starts salling tokens)
- `pub fn buy_tokens(tokens_cnt: u128)` (sender buys tokens_cnt tokens)
- `fn get_balance()` (returns the number of remaining tokens)
- `fn update_price()` (checks current timestamp and updates price if needed)
- `fn in_process()` (checks if ico wasn't ended and get_balance() > 0)
