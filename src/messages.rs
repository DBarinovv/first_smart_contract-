use ft_io::*;
use gstd::{msg, ActorId};
// use primitive_types::U256;

pub async fn transfer_tokens(token_id: &ActorId, from: &ActorId, to: &ActorId, amount: u128) {
    let _transfer_response: Event = msg::send_and_wait_for_reply(
        *token_id,
        Action::Transfer {
            from: *from,
            to: *to,
            amount,
        },
        0,
    )
    .unwrap()
    .await
    .expect("Error in transfer");
}

pub async fn balance(token_id: &ActorId, account: &ActorId) -> u128 {
    let balance_response: Event =
        msg::send_and_wait_for_reply(*token_id, Action::BalanceOf(*account), 0)
            .unwrap()
            .await
            .expect("Error in balance");

    if let Event::Balance(balance_response) = balance_response {
        balance_response
    } else {
        0
    }
}