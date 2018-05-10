use bincode::{Error as BError};
use tera::{Error as TError};
use walkdir::{Error as WDError};

#[derive(Debug)]
pub struct StateError {
    pub msg: String,
}

impl StateError {
    pub fn new<T: ToString>(msg: T) -> StateError {
        StateError {
            msg: msg.to_string(),
        }
    }
}

pub type StateResult = Result<String, StateError>;

impl From<WDError> for StateError {
    fn from(other: WDError) -> StateError {
        StateError {
            msg: format!("{:?}", other)
        }
    }
}

impl From<::std::io::Error> for StateError {
    fn from(other: ::std::io::Error) -> StateError {
        StateError {
            msg: format!("{:?}", other)
        }
    }
}

impl From<String> for StateError {
    fn from(other: String) -> StateError {
        StateError {
            msg: other,
        }
    }
}


impl From<TError> for StateError {
    fn from(other: TError) -> StateError {
        StateError::new(other.kind().description())
    }
}

impl From<BError> for StateError {
    fn from(other: BError) -> StateError {
        let kind = *other;
        StateError::new(format!("Bincode Error: {:?}", kind))
    }
}