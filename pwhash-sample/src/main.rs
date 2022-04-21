use pwhash::bcrypt;
use pwhash::bcrypt::{BcryptSetup, BcryptVariant};

fn main() {
    let password = "test_password";
    let hashed1 = bcrypt::hash_with(
        BcryptSetup {
            salt: Some("hogeeeeeeeeeeeeeeeee"),
            variant: Some(BcryptVariant::V2y),
            ..Default::default()
        },
        password,
    ).unwrap();
    let hashed2 = bcrypt::hash(password).unwrap();
    assert!(bcrypt::verify("test_password", &hashed1));
    assert!(bcrypt::verify("test_password", &hashed2));
    println!("{}", hashed1);
    println!("{}", hashed2);
}
