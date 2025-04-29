use crate::{types::*, Result};
use subversive_crypto::address::Address;
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Debug, Default)]
pub struct BalanceManager {
    balances: RwLock<HashMap<Address, Balance>>,
}

impl BalanceManager {
    pub fn new() -> Self {
        Self {
            balances: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get_balance(&self, address: &Address) -> Balance {
        let balances = self.balances.read().await;
        balances
            .get(address)
            .cloned()
            .unwrap_or_else(|| Balance::new(Amount(0)))
    }

    pub async fn update_balance(&self, address: Address, amount: Amount) -> Result<()> {
        let mut balances = self.balances.write().await;
        let balance = balances.entry(address).or_insert_with(|| Balance::new(Amount(0)));
        balance.amount = amount;
        balance.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Ok(())
    }
}
