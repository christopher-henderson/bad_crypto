use std::fs;

extern crate time;
extern crate bad_crypto;

use bad_crypto::{encrypt, decrypt};


const ORIGINAL: &str = "/Users/chris/Documents/personal/rust/projects/bad_crypto/ci_real_word_deployments.mov";
const CIPHER: &str = "/Users/chris/Documents/personal/rust/projects/bad_crypto/out.mov";

fn dec() {
	let mut plaintext = fs::read(CIPHER).unwrap();
    let key = 21234111;
    let start = time::PreciseTime::now();
	decrypt(&mut plaintext, key);
	let end = time::PreciseTime::now();
	println!("finished in {} seconds", start.to(end));
	fs::write("/Users/chris/Documents/personal/rust/projects/bad_crypto/decrypted.mov", plaintext).unwrap();
}

fn enc() {
	let mut plaintext = fs::read(ORIGINAL).unwrap();
    let key = 21234111;
    let start = time::PreciseTime::now();
	encrypt(&mut plaintext, key);
	let end = time::PreciseTime::now();
	println!("finished in {} seconds", start.to(end));
	fs::write("/Users/chris/Documents/personal/rust/projects/bad_crypto/out.mov", plaintext).unwrap();
}

fn main() {
	enc();
	dec();
}
