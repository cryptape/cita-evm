use cita_evm as evm;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    Evm(evm::Error),
    Secp256k1(secp256k1::Error),
    IO(io::Error),
    Str(String),
    NotEnoughBaseGas,
    NotEnoughBalance,
    InvalidNonce,
    ContractAlreadyExist,
    ExccedMaxCodeSize,
    ExccedMaxBlockGasLimit,
    ExccedMaxCallDepth,
    CreateInStaticCall,
    Trie(String),
    RLP(rlp::DecoderError),
    DB(String),
    NotFound,
    BalanceError,
}

impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Evm(e) => return write!(f, "{}", e),
            Error::Secp256k1(e) => return write!(f, "{:?}", e),
            Error::IO(e) => return write!(f, "{:?}", e),
            Error::Str(e) => return write!(f, "{:?}", e),
            Error::NotEnoughBaseGas => return write!(f, "NotEnoughBaseGas"),
            Error::NotEnoughBalance => return write!(f, "NotEnoughBalance"),
            Error::InvalidNonce => return write!(f, "InvalidNonce"),
            Error::ContractAlreadyExist => return write!(f, "ContractAlreadyExist"),
            Error::ExccedMaxCodeSize => return write!(f, "ExccedMaxCodeSize"),
            Error::ExccedMaxBlockGasLimit => return write!(f, "ExccedMaxBlockGasLimit"),
            Error::ExccedMaxCallDepth => return write!(f, "ExccedMaxCallDepth"),
            Error::CreateInStaticCall => return write!(f, "CreateInStaticCall"),
            Error::Trie(e) => return write!(f, "state trie: {}", e),
            Error::RLP(e) => return write!(f, "state rlp: {}", e),
            Error::DB(e) => return write!(f, "state db: {}", e),
            Error::NotFound => return write!(f, "state: not found"),
            Error::BalanceError => return write!(f, "state: balance error"),
        };
    }
}

impl From<evm::Error> for Error {
    fn from(error: evm::Error) -> Self {
        Error::Evm(error)
    }
}

impl From<secp256k1::Error> for Error {
    fn from(error: secp256k1::Error) -> Self {
        Error::Secp256k1(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IO(error)
    }
}

impl From<cita_trie::TrieError> for Error {
    fn from(error: cita_trie::TrieError) -> Self {
        Error::Trie(format!("{}", error))
    }
}

impl From<rlp::DecoderError> for Error {
    fn from(error: rlp::DecoderError) -> Self {
        Error::RLP(error)
    }
}
