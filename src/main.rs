use std::io;
use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha2::{Sha256, Sha512};

fn main() {
    loop {
        println!("Digite a senha (ou 'quit' para sair):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
        let password = input.trim();

        if password == "quit" || password == "exit" {
            println!("Saindo da aplicação!");
            break;
        }

        println!("MD5: {}", hash_md5(password));
        println!("SHA256: {}", hash_sha256(password));
        println!("SHA512: {}", hash_sha512(password));
    }
}

fn hash_md5(password: &str) -> String {
    let mut hasher = Md5::new();
    hasher.input_str(password);
    hasher.result_str()
}

fn hash_sha256(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(password);
    hasher.result_str()
}

fn hash_sha512(password: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.input_str(password);
    hasher.result_str()
}
