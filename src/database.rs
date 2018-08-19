use std::collections::{hash_map, HashMap};
use std::io::{BufRead, Write};
use std::sync::{Arc, RwLock};

use failure::{Error, ResultExt};
use multi_mut::HashMapMultiMut;
use serde_derive::{Deserialize, Serialize};
use serde_json;

use crate::common::{Coins, UserId, UserSecret};
use crate::errors::{TransferError, TransferErrorKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_deserializing)]
    user_id: UserId,
    #[serde(skip_serializing)]
    secret: UserSecret,
    full_name: String,
    #[serde(skip_deserializing)]
    balance: Coins,
}

impl User {
    pub fn get_user_id(&self) -> UserId {
        self.user_id
    }
}

#[derive(Debug, Clone)]
pub struct UsersCollection {
    pub in_memory_storage: HashMap<UserId, User>,
}

impl UsersCollection {
    pub fn load_from_file<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Self> {
        let file = std::fs::File::open(path)?;
        let mut users_collection = Self {
            in_memory_storage: serde_json::from_reader(file)?,
        };
        for (user_id, user) in users_collection.in_memory_storage.iter_mut() {
            user.user_id = *user_id;
            if user_id == &UserId::master() {
                user.balance = Coins::new(1000000000);
            }
        }
        Ok(users_collection)
    }

    fn get(&self, user_id: &UserId) -> Option<&User> {
        self.in_memory_storage.get(user_id)
    }

    fn values(&self) -> hash_map::Values<UserId, User> {
        self.in_memory_storage.values()
    }

    fn validate_transfer<'a>(
        &'a mut self,
        transfer: &'a Transfer,
    ) -> Result<ValidatedTransfer<'a>, TransferError> {
        let (from_user, to_user) = self
            .in_memory_storage
            .get_pair_mut(&transfer.from, &transfer.to)
            .ok_or_else(|| TransferErrorKind::NoUserError(vec![transfer.from, transfer.to]))?;
        let new_from_user_balance =
            from_user
                .balance
                .checked_sub(transfer.amount)
                .ok_or_else(|| TransferErrorKind::NotEnoughCoinsError {
                    balance: from_user.balance,
                    amount: transfer.amount,
                })?;
        let new_to_user_balance =
            to_user
                .balance
                .checked_add(transfer.amount)
                .ok_or_else(|| TransferErrorKind::TooManyCoinsError {
                    balance: to_user.balance,
                    amount: transfer.amount,
                })?;
        Ok(ValidatedTransfer {
            transfer,
            from: from_user,
            to: to_user,
            new_from_user_balance,
            new_to_user_balance,
        })
    }

    fn apply_transfers(&mut self, transfers: &[Transfer]) -> Result<(), Error> {
        for transfer in transfers {
            self.validate_transfer(transfer)?.commit();
        }
        Ok(())
    }
}

#[derive(Debug)]
struct PersistentAppendOnlyStorage {
    storage: std::fs::File,
}

impl PersistentAppendOnlyStorage {
    fn new<P: AsRef<std::path::Path>>(persistent_storage_filepath: P) -> std::io::Result<Self> {
        Ok(Self {
            storage: std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(persistent_storage_filepath)?,
        })
    }

    fn append(&mut self, transfer: &Transfer) -> Result<(), Error> {
        let mut serialized_transfer = serde_json::to_string(&transfer)?;
        serialized_transfer.push_str("\n");
        self.storage.write_all(serialized_transfer.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Transfer {
    date: std::time::SystemTime,
    from: UserId,
    to: UserId,
    amount: Coins,
}

impl Transfer {
    fn new(from: UserId, to: UserId, amount: Coins) -> Self {
        Self { date: std::time::SystemTime::now(), from, to, amount }
    }
}

#[derive(Debug)]
struct ValidatedTransfer<'a> {
    transfer: &'a Transfer,
    from: &'a mut User,
    to: &'a mut User,
    new_from_user_balance: Coins,
    new_to_user_balance: Coins,
}

impl ValidatedTransfer<'a> {
    fn commit(&mut self) {
        self.from.balance = self.new_from_user_balance;
        self.to.balance = self.new_to_user_balance;
    }
}

#[derive(Debug)]
pub struct TransfersCollection {
    in_memory_storage: Vec<Transfer>,
    persistent_storage: PersistentAppendOnlyStorage,
}

impl TransfersCollection {
    pub fn load_from_file<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Self> {
        let mut in_memory_storage = Vec::new();
        let file = std::fs::File::open(&path)?;
        for line in std::io::BufReader::new(file).lines() {
            in_memory_storage.push(serde_json::from_str::<Transfer>(&line?)?);
        }
        Ok(Self {
            in_memory_storage,
            persistent_storage: PersistentAppendOnlyStorage::new(path)?,
        })
    }

    fn append(&mut self, transfer: Transfer) -> Result<(), TransferError> {
        self.persistent_storage
            .append(&transfer)
            .with_context(|_| {
                TransferErrorKind::IOError("Persistent storage could not record a transfer.".into())
            })?;
        self.in_memory_storage.push(transfer);
        Ok(())
    }
}

type Records<T> = Arc<RwLock<T>>;

pub struct DB {
    users: Records<UsersCollection>,
    users_by_secret: Records<HashMap<UserSecret, UserId>>,
    transfers: Records<TransfersCollection>,
}

impl DB {
    pub fn new(mut users: UsersCollection, transfers: TransfersCollection) -> Result<Self, Error> {
        users.apply_transfers(&transfers.in_memory_storage)?;
        let mut users_by_secret = HashMap::new();
        for user in users.values() {
            users_by_secret.insert(user.secret.clone(), user.user_id);
        }

        Ok(Self {
            users: Arc::new(RwLock::new(users)),
            users_by_secret: Arc::new(RwLock::new(users_by_secret)),
            transfers: Arc::new(RwLock::new(transfers)),
        })
    }

    pub fn get_user(&self, user_id: UserId) -> Option<User> {
        Some(self.users.read().ok()?.get(&user_id)?.clone())
    }

    pub fn get_user_by_secret(&self, user_secret: &UserSecret) -> Option<User> {
        self.get_user(*self.users_by_secret.read().ok()?.get(user_secret)?)
    }

    pub fn get_users(&self) -> UsersCollection {
        (*self.users.read().unwrap()).clone()
    }

    pub fn get_transfers(&self) -> Vec<Transfer> {
        (*self.transfers.read().unwrap()).in_memory_storage.clone()
    }

    pub fn transfer(&self, from: UserId, to: UserId, amount: Coins) -> Result<(), TransferError> {
        let mut users = match self.users.write() {
            Err(_) => {
                return Err(TransferErrorKind::UnexpectedError(
                    "Could not lock DB.users to perform transfer.".into(),
                ).into())
            }
            Ok(users) => users,
        };
        let transfer = Transfer::new(from, to, amount);
        let mut validated_transfer = users.validate_transfer(&transfer)?;
        match self.transfers.write() {
            Err(_) => {
                return Err(TransferErrorKind::UnexpectedError(
                    "Could not lock DB.transfers to perform transfer.".into(),
                ).into())
            }
            Ok(mut transfers) => transfers.append(validated_transfer.transfer.clone())?,
        }
        validated_transfer.commit();
        Ok(())
    }
}
