use std::env::{self};

pub fn get() -> String {
    if let Ok(user) = env::var("USER") {
        return user;
    } else {
        return String::from("UNKNOWN");
    }
}
