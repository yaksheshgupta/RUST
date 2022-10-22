// creating a basic DEX
use std::collections::HashMap;
use std::io;
// tokens available

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
enum TokensGiven {
    Sol,
    Eth,
    Dodge,
    Shiba,
    USDT,
    Yakshesh,
}
// user wallet struct
#[derive(Debug)]
struct Wallet {
    address: String,
    balance: HashMap<TokensGiven, f32>,
}
#[derive(Debug)]
struct Balance {
    token: TokensGiven,
    balance: f32,
}
impl Wallet {
    fn create_wallet(address: String, balance: HashMap<TokensGiven, f32>) -> Self {
        println!("Wallet Created");
        Self { address, balance }
    }
    fn check_balance(&self) {
        println!("{:?}", self.balance);
    }
}

// return marcket
impl TokensGiven {
    fn return_tk(name: &str) -> Self {
        match name {
            "sol" => TokensGiven::Sol,
            "Dodge" => TokensGiven::Dodge,
            "eth" => TokensGiven::Eth,
            "usdt" => TokensGiven::USDT,
            "shiba" => TokensGiven::Shiba,
            "yakshesh" => TokensGiven::Yakshesh,
            _ => TokensGiven::Yakshesh,
        }
    }

    fn mkget_price(&self) -> f32 {
        match &self {
            TokensGiven::Sol => 32.0,
            TokensGiven::Eth => 50.0,
            TokensGiven::Dodge => 5.0,
            TokensGiven::Shiba => 1.0,
            TokensGiven::USDT => 1.0,
            TokensGiven::Yakshesh => 100.0,
        }
    }
    fn print_mkprice() {
        println!("Sol --- {}", Self::mkget_price(&TokensGiven::Sol));
        println!("Eth --- {}", Self::mkget_price(&TokensGiven::Eth));
        println!("Dodge --- {}", Self::mkget_price(&TokensGiven::Dodge));
        println!("Shiba --- {}", Self::mkget_price(&TokensGiven::Shiba));
        println!("Yakshesh --- {}", Self::mkget_price(&TokensGiven::Yakshesh));
        println!("USDT --- {}", Self::mkget_price(&TokensGiven::USDT));
        println!("")
        // for tk in &self::TokensGiven {
        //     println!("token-{:?} price{:?}",tk,TokensGiven::mkget_price(tk));
        // }
    }
    fn buy_token(self, amt: f32, balance: &mut HashMap<TokensGiven, f32>) {
        let val = balance.get(&TokensGiven::USDT).unwrap();
        let token_price = Self::mkget_price(&self);
        let calculated_price = amt * token_price;
        if val >= &calculated_price {
            // balance.entry(TokensGiven::USDT).or_insert(*val-calculated_price);
            // balance.entry(self).or_insert(amt);
            balance.insert(TokensGiven::USDT, val - &calculated_price);
            if balance.contains_key(&self) {
                let prev_bal = balance.get(&self).unwrap();
                balance.insert(self, &amt + prev_bal);
            } else {
                balance.insert(self, amt);
            }
            println!("Transaction done");
        } else {
            println!("insufficient balance");
        }
    }
    fn sell_tkn(self,amt:f32,balance:&mut HashMap<TokensGiven, f32>) {
        let current_balance= balance.get(&self).unwrap();
        let token_price = Self::mkget_price(&self);
        let calculated_price = amt * token_price;
        if current_balance>=&amt {
            balance.insert(self, current_balance-&amt);
            if *balance.get(&self).unwrap()==0.0 {
               
            }
            balance.insert(TokensGiven::USDT, balance.get(&TokensGiven::USDT).unwrap()+calculated_price);
            println!("Transaction successfull");
        }else {
            println!("Transaction unsuccessfull");
        }
    }
}
fn main() {
    println!("Please enter the address");
    let mut user_wallet = String::new();
    io::stdin().read_line(&mut user_wallet);
    let mut initial_balance_ofuser = HashMap::new();
    initial_balance_ofuser.insert(TokensGiven::USDT, 1000.0);
    let mut user_wallet_instance = Wallet::create_wallet(user_wallet, initial_balance_ofuser);
    loop {
        println!("1->check balance");
        println!("2-> see marcket");
        println!("3->buy tokens");
        println!("4->sell tokens");
        println!("6->To exit");
        let mut val = String::new();
        io::stdin().read_line(&mut val);
        let int_val: i32 = val.trim().parse().unwrap();
        match int_val {
            1 => user_wallet_instance.check_balance(),
            2 => {
                TokensGiven::print_mkprice();
            }
            3 => {
                TokensGiven::print_mkprice();
                println!("Which token you want to buy???");
                let mut tk_name = String::new();
                io::stdin().read_line(&mut tk_name);
                let buying_token = TokensGiven::return_tk(&tk_name.trim());
                println!("Amount of {} you want to sell-?", tk_name);
                let mut amount_oftkn = String::new();
                io::stdin().read_line(&mut amount_oftkn);
                let amountoftkn = amount_oftkn.trim().parse().unwrap();
                TokensGiven::buy_token(
                    buying_token,
                    amountoftkn,
                    &mut user_wallet_instance.balance,
                );
            }
            4 => {
                TokensGiven::print_mkprice();
                println!("Which token you want to sell???");
                let mut tk_name = String::new();
                io::stdin().read_line(&mut tk_name);
                let buying_token = TokensGiven::return_tk(&tk_name.trim());
                println!("Amount of {} you want to sell-?", tk_name);
                let mut amount_oftkn = String::new();
                io::stdin().read_line(&mut amount_oftkn);
                let amountoftkn = amount_oftkn.trim().parse().unwrap();
                TokensGiven::sell_tkn(
                    buying_token,
                    amountoftkn,
                    &mut user_wallet_instance.balance,
                );
            }
            6 => break,
            _ => {
                println!("sahi value daal na bhai")
            }
        }
    }
}
