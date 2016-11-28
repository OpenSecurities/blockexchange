
extern crate openssl;

use std::fs::File;
use std::fs::{self, DirBuilder};
use std::io::prelude::*;
use std::os::unix::fs::PermissionsExt;

use self::openssl::hash::MessageDigest;
use self::openssl::pkey::PKey;
use self::openssl::rsa::Rsa;
use self::openssl::x509::X509Generator;
use self::openssl::x509::extension::{Extension, KeyUsageOption};

use config;

/// Helper function for creating a cert/key using filename
fn create_cert(years: u32, org: &str, filename: &str, key_options: Vec<KeyUsageOption>) {

    create_auth_dir();

    // Configure x509
    let rsa = Rsa::generate(4096).unwrap();
    let pkey = PKey::from_rsa(rsa).unwrap();

    let gen = X509Generator::new()
        .set_valid_period(365*years)
        .add_name("CN".to_owned(), org.to_owned())
        .set_sign_hash(MessageDigest::sha256())
        .add_extension(Extension::KeyUsage(key_options));

    let cert = gen.sign(&pkey).unwrap().to_pem();
    let pkey_pem = pkey.private_key_to_pem();

    // Write the certificate
    let cert_file = config::get_auth_dir() + "/" + filename + ".pem";
    let mut f = File::create(&cert_file).expect("Unable to create certificate file.");
    for line in cert.unwrap() {
        f.write(&[line]).expect("Error writing certificate!");
    }
    let metadata = f.metadata().expect("Unable to get file metadata");
    let mut perms = metadata.permissions();
    perms.set_mode(0o400);
    fs::set_permissions(&cert_file, perms).expect("Unable to set certificate file to read-only");

    // Write the private key
    let pkey_file = config::get_auth_dir() + "/" + filename + ".key";
    let mut f = File::create(&pkey_file).expect("Unable to create the private key");
    for line in pkey_pem.unwrap() {
        f.write(&[line]).expect("Error writing private key");
    }
    let metadata = f.metadata().expect("Unable to get file metadata");
    let mut perms = metadata.permissions();
    perms.set_mode(0o400);
    fs::set_permissions(&pkey_file, perms).expect("Unable to set CA file to read-only");
}

/// Build client authentication certificates
pub fn build_client_auth(years: u32, org: &str) {
    println!("Building authentication.");
    create_auth_dir();
    create_cert(years, org, "client", vec![KeyUsageOption::DigitalSignature]);
}

/// Build CA certificate for signing all client certificates.
/// NOTE: by design the CA certificate will not be able to sign transactions. It is
/// only used to authorize/de-authorize client certificates.
pub fn build_ca(years: u32, org: &str) {
    println!("Creating CA");

    create_auth_dir();
    create_cert(years, org, "ca", vec![KeyUsageOption::KeyCertSign, KeyUsageOption::CRLSign]);
}

/// Insure the auth directory exists
fn create_auth_dir() {
    let path = config::get_auth_dir();
    DirBuilder::new()
        .recursive(true)
        .create(path).unwrap();
}
