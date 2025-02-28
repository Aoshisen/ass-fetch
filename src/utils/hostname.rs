use hostname;
use std::ffi::OsString;
use std::io;
pub fn get() -> io::Result<OsString> {
    hostname::get()
}
