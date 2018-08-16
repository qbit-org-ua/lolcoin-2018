use failure::{Backtrace, Context, Fail};

use crate::common::{Coins, UserId};

#[derive(Debug, Fail)]
pub enum TransferErrorKind {
    #[fail(
        display = "There is missing one or more users with id {:?}",
        0
    )]
    NoUserError(Vec<UserId>),
    #[fail(
        display = "There is not enough coins (requested -{}) on the balance ({})",
        amount,
        balance
    )]
    NotEnoughCoinsError { balance: Coins, amount: Coins },
    #[fail(
        display = "There are too many coins (requested +{}) on the balance ({})",
        amount,
        balance
    )]
    TooManyCoinsError { balance: Coins, amount: Coins },
    #[fail(display = "IO error: {}", 0)]
    IOError(String),
    #[fail(display = "Unexpected error: {}", 0)]
    UnexpectedError(String),
}

impl From<Context<TransferErrorKind>> for TransferError {
    fn from(context_error_kind: Context<TransferErrorKind>) -> Self {
        Self {
            inner: context_error_kind,
        }
    }
}

impl From<TransferErrorKind> for TransferError {
    fn from(error_kind: TransferErrorKind) -> Self {
        Self {
            inner: Context::new(error_kind),
        }
    }
}

#[derive(Debug)]
pub struct TransferError {
    inner: Context<TransferErrorKind>,
}

impl std::fmt::Display for TransferError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.inner, f)
    }
}

impl Fail for TransferError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}
