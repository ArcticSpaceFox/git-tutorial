use std::{
    fs::File,
    io::{Read, Write},
};

use rand::rngs::OsRng;
use rsa::{pkcs8::DecodePublicKey, PaddingScheme, PublicKey, RsaPublicKey};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Options {
    /// First name
    #[structopt(short = "f", long = "first")]
    first: String,
    /// Usertag (aka "Nutzerkürzel")
    #[structopt(short = "u", long = "usertag")]
    usertag: String,
    /// message to send
    #[structopt(short = "m", long = "message")]
    message: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Options::from_args();

    let mut rng = OsRng;

    println!("✏️ - Creating message...");

    let mut public_key_file = File::open("public_key.pem")?;
    let mut public_key_string = String::new();
    public_key_file.read_to_string(&mut public_key_string)?;
    let public_key = RsaPublicKey::from_public_key_pem(&public_key_string)?;

    // Encrypt
    let cipher_text = public_key
        .encrypt(
            &mut rng,
            PaddingScheme::new_pkcs1v15_encrypt(),
            &args.message.as_bytes(),
        )
        .expect("failed to encrypt");

    println!("🔐 - Message encrypted with RSA");
    // ciphertext to hex string
    let cipher_text_hex = hex::encode(cipher_text);

    // print last, first, usertag, cipher text to messages/usertag.txt
    let mut message_file = File::create("messages/".to_string() + &args.usertag + ".txt")?;
    message_file.write_all(
        &format!("#{}, {} - {}", args.usertag, args.first, cipher_text_hex).as_bytes(),
    )?;

    println!("📝 - Message saved to messages/{}.txt", args.usertag);
    println!("✔️ - Done!\n");
    println!("💾 - Now commit the message with git!");

    Ok(())
}
