use serde_derive::Deserialize;

use crate::common::{Coins, UserId};

#[derive(Debug, Deserialize)]
pub struct TransferData {
    pub to: UserId,
    pub amount: Coins,
}
