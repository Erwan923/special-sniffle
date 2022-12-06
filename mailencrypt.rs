extern crate base64;
extern crate rust_crypto;

use rust_crypto::aes::Aes256;
use rust_crypto::blockmodes::{CbcDecryptor, CbcEncryptor};
use rust_crypto::buffer::{RefReadBuffer, RefWriteBuffer, ReadBuffer, WriteBuffer};
use rust_crypto::symmetriccipher::{Encryptor, Decryptor};

fn encrypt_email(email: &str, password: &str) -> String {
    // Convertir le mot de passe en clé de chiffrement
    let key = password_to_key(password);

    // Convertir le mail en bytes
    let email_bytes = email.as_bytes();

    // Générer un vecteur d'initialisation aléatoire
    let iv = generate_random_iv();

    // Chiffrer les bytes du mail en utilisant la clé et le vecteur d'initialisation
    let encrypted_bytes = encrypt(email_bytes, &key, &iv);

    // Encoder les bytes chiffrés en base64 pour pouvoir les envoyer dans un mail
    let encrypted_email = base64::encode(&encrypted_bytes);

    return encrypted_email;
}

fn decrypt_email(encrypted_email: &str, password: &str) -> String {
    // Convertir le mot de passe en clé de chiffrement
    let key = password_to_key(password);

    // Décoder les bytes chiffrés en base64
    let encrypted_bytes = base64::decode(encrypted_email).unwrap();

    // Récupérer le vecteur d'initialisation du début du message chiffré
    let iv = get_iv_from_encrypted_message(&encrypted_bytes);

    // Déchiffrer les bytes en utilisant la clé et le vecteur d'initialisation
    let decrypted_bytes = decrypt(&encrypted_bytes, &key, &iv);

    // Convertir les bytes déchiffrés en chaîne de caractères
    let decrypted_email = String::from_utf8(decrypted_bytes).unwrap();

    return decrypted_email;
}

fn password_to_key(password: &str) -> Vec<u8> {
    // Convertir le mot de passe en bytes
    let password_bytes = password.as_bytes();

    // Répéter le mot de passe jusqu'à ce qu'il fasse 256 bits
    let mut key = Vec::new();
    for i in 0..(256 / password_bytes.len() + 1) {
        key.extend_from_slice(password_bytes);
    }
    key.truncate(256 / 8);

    return key;
}

fn generate_random_iv() -> Vec<u8> {
    // Générer un vecteur d'initialisation aléatoire de 128 bits
    let mut iv = Vec::new();
