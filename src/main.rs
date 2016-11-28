mod auth;
mod config;

fn main() {
    auth::build_ca(10, "Deberon LLC");
    auth::build_client_auth(10, "Derek Anderson");
}
