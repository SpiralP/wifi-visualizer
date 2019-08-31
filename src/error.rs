use std::result::Result as StdResult;

pub use failure::{bail, err_msg};

pub type Error = failure::Error;
pub type Result<T> = StdResult<T, Error>;
