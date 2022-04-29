#![no_std]

use gstd::{msg, ActorId, Box};
use codec::{Decode, Encode};

use primitive_types::U256;

trait IERC20Token {
    fn balance_of(&self, owner: ActorId) -> U256;
    fn transfer(&self, to: ActorId, amount: U256) -> bool;
    fn decimals(&self) -> U256;
}
struct TokenSale {
    token_contract: Box<dyn IERC20Token>,
    price: U256, 
    owner: ActorId,
    tokens_sold: U256, 
    token_id: ActorId,
}

static mut TOKEN_SALE: Option<TokenSale> = None;
const TOKEN_ID: ActorId = ActorId::new([2u8; 32]);

impl TokenSale {
    pub fn new(token_contract: Box<dyn IERC20Token>, price: U256, token_id: ActorId) -> TokenSale {
        TokenSale {
            token_contract,
            price,
            owner: msg::source(),
            tokens_sold: U256::from(0), 
            token_id,
        }
    }

    pub fn buy_tokens(&mut self, tokens_cnt: U256)  {

        let (res, overflow) = tokens_cnt.overflowing_mul(self.price);
        if !overflow {
            panic!("Overflowing multiplication")
        }

        if msg::load::<U256>().unwrap() != res {
            panic!("Wrong msg value")
        }

        let (scaled, overflow) = tokens_cnt.overflowing_mul((U256::from(10)).pow(self.token_contract.decimals()));
        if !overflow {
            panic!("Overflowing multiplication")
        }

        if self.token_contract.balance_of(self.token_id) < scaled {
            panic!("Not enough money")
        }

        msg::reply(tokens_cnt, 0).unwrap();
        self.tokens_sold += tokens_cnt;

        if !self.token_contract.transfer(msg::source(), scaled) {
            panic!("token_contract transfer")
        }
    }

    pub fn end_sale(&self) {
        if msg::source() != self.owner {
            panic!("Wrong owner")
        }

        if !self.token_contract.transfer(self.owner, self.token_contract.balance_of(self.token_id)) {
            panic!("token_contract transfer");
        }
    }
}


struct Helper{}
impl IERC20Token for Helper {
    fn balance_of(&self, owner: ActorId) -> U256 {
        U256::from(0)
    }

    fn transfer(&self, to: ActorId, amount: U256) -> bool {
        true
    }

    fn decimals(&self) -> U256 {
        U256::from(0)
    }
}

#[derive(Debug, Decode, Encode)]
pub enum Action {
    Send(U256),
    EndSale
}

#[no_mangle]
pub unsafe extern "C" fn handle() {
    let action = msg::load().expect("Could not load");
    let tk_sale: &mut TokenSale = 
        TOKEN_SALE.get_or_insert(TokenSale::new(Box::new(Helper {}), U256::from(10), TOKEN_ID));

    match action {
        Action::Send(value) => {
            tk_sale.buy_tokens(value)
        }
        Action::EndSale => {
            tk_sale.end_sale()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {}