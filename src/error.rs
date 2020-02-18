use redis::RedisError;

/// Common error type for this crate.
#[derive(Debug)]
pub enum RedisGraphError {
    /// Any error originating from the `redis` crate.
    RedisError(RedisError),
    /// Result of a miscommunication between this crate and the database.
    ///
    /// *This should never happen. If it does, please open an issue at https://github.com/malte-v/redisgraph-rs/issues/new .*
    ServerTypeError(String),
    /// Returned if the data you requested is of a different type
    /// than the data returned by the database.
    ClientTypeError(String),

    /// Returned if a label name was not found in the graph's internal registry.
    ///
    /// This error is taken care of by the implementation and should never reach your code.
    LabelNotFound,
    /// Returned if a relationship type name was not found in the graph's internal registry.
    ///
    /// This error is taken care of by the implementation and should never reach your code.
    RelationshipTypeNotFound,
    /// Returned if a property key name was not found in the graph's internal registry.
    ///
    /// This error is taken care of by the implementation and should never reach your code.
    PropertyKeyNotFound,

    /// Returned if you requested a [`String`](https://doc.rust-lang.org/std/string/struct.String.html) and the database responded with bytes that are invalid UTF-8.
    ///
    /// If you don't care about whether the data is valid UTF-8, consider requesting a [`RedisString`](../result_set/struct.RedisString.html) instead.
    InvalidUtf8,
}

impl From<RedisError> for RedisGraphError {
    fn from(error: RedisError) -> RedisGraphError {
        RedisGraphError::RedisError(error)
    }
}

/// Common result type for this crate.
pub type RedisGraphResult<T> = Result<T, RedisGraphError>;

#[doc(hidden)]
#[macro_export]
macro_rules! client_type_error {
    ($($arg:tt)*) => {
        Err($crate::RedisGraphError::ClientTypeError(format!($($arg)*)))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! server_type_error {
    ($($arg:tt)*) => {
        Err($crate::RedisGraphError::ServerTypeError(format!($($arg)*)))
    };
}
