use hostname;
use std::io;
pub fn get() -> io::Result<String> {
    match hostname::get() {
        Ok(h) => Ok(h.to_string_lossy().trim().to_string()),
        Err(e) => Err(e),
    }
}
