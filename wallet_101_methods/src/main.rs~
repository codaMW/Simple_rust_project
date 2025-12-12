#[derive(Debug)]
#[allow(unused)]

struct Wallet {
    pub_key: String,
    balance: f64,
}

#[allow(unused)]

impl Wallet {
    fn get_balance(&self) -> f64 {
        self.balance
    }
    
    fn deposit(&self, input: u32) -> f64 {

        let updated_balance: f64 = (input as f64 / 100_000_000.0) + self.get_balance();

        updated_balance
    }

    fn withdraw(&self, deduct: u32) -> f64 {

        let ded_amount: f64 = deduct as f64 / 100_000_000.0;

        if self.get_balance() >= ded_amount {
            let withdraw_balance: f64 = self.get_balance() - ded_amount;

            return withdraw_balance;
        } else if self.get_balance() < ded_amount {
            
            return 0.000;
        } else {
            return 0.0000;
        }
    }
}

#[allow(unused)]

fn main() {

    let blink = Wallet {
        pub_key: String::from("codaMW"),
        balance: 0.003570,
    };


    let deposit_amount: u32 = 750_000;
    let withdraw_amount: u32 = 250_000;
    
    let balance_b4_depo: f64 = blink.get_balance();
    
    println!("\nWallet: {}\nBalance before deposit: {:.6}\n",blink.pub_key, balance_b4_depo);

    let balance_after_depo: f64 = blink.deposit(deposit_amount);

    let balance_after_with: f64 = blink.withdraw(withdraw_amount);

    println!("Wallet: {}\nBalance after deposit: {:.6}\n",blink.pub_key, balance_after_depo);

    println!("Wallet: {}\nBalance after withdraw: {:.6}\n", blink.pub_key, balance_after_with); 
    
}
