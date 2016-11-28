
use std::env;

/// Fetch an environment variable, but return a default value if not found
fn get_env(key: String, default: String) -> String {

    let found = env::var_os(key);

    if found.is_some() {
        found.unwrap().into_string().unwrap()
    } else {
        default
    }
}

/// Return the configured authentication directory, with a sane default
pub fn get_auth_dir() -> String {
    let home_dir = get_env("HOME".to_string(), "/var/lib/blockexchange".to_string());

    get_env("BE_AUTH_DIR".to_string(), home_dir + &"/.blockexchange/auth")
}
