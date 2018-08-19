use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new(uuid: Uuid) -> Self {
        UserId(uuid)
    }

    pub fn master() -> Self {
        Self::new(Uuid::nil())
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::master()
    }
}

impl PartialEq for UserId {
    fn eq(&self, other: &UserId) -> bool {
        self.0.eq(&other.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserSecret(String);

impl UserSecret {
    pub fn new(secret: String) -> Self {
        UserSecret(secret)
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Coins(u64);

impl Coins {
    pub fn new(coins: u64) -> Self {
        Coins(coins)
    }

    pub fn checked_add(self, rhs: Coins) -> Option<Coins> {
        Some(Coins(self.0.checked_add(rhs.0)?))
    }

    pub fn checked_sub(self, rhs: Coins) -> Option<Coins> {
        Some(Coins(self.0.checked_sub(rhs.0)?))
    }
}

impl std::fmt::Display for Coins {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} coins", self.0)
    }
}

impl Default for Coins {
    fn default() -> Self {
        Coins::new(0)
    }
}
