use std::result::Result as StdResult;

pub type Error = failure::Error;
pub type Result<T> = StdResult<T, Error>;
