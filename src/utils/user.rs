use std::env::{self, VarError};

pub fn get() -> Result<String, VarError> {
    env::var("USER")
}
