#[derive(Debug)]

struct Wallet {
    pub_key: String,
    balance: f64,
}

#[allow(dead_code)]

impl Wallet {
    fn get_owner(&self) -> String {
        self.pub_key.clone()
    }
    fn get_balance(&self) -> f64 {
        self.balance
    }
    
    fn sats_converter(&self) ->f64 {
        self.balance * 100_000_000 as f64
    }
}



fn main() {
    
    let satoshi = Wallet {
        pub_key: String::from("np0k1233409zk0o980"),
        balance: 0.00057165,
    };

    println!("{}", satoshi.pub_key);

    println!("balance = {:.2} sats", satoshi.sats_converter());



}
