use hostname;
pub fn get() -> String {
    if let Ok(hostname) = hostname::get() {
        return hostname.to_string_lossy().into_owned();
    } else {
        return String::from("UNKNOWN");
    }
}
