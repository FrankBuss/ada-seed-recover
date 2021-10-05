use bip39::*;
use bitcoin_hashes::{sha256, Hash};

// simplified copy of Mnemonic::parse_in, because it doesn't allow 15 word seeds
pub fn parse_15_words(all_words: &[&str], passphrase: Vec<String>) -> bool {
    assert_eq!(passphrase.len(), 15);
    let mut words = [0; 15];
    let mut bits = [false; 15 * 11];
    for (i, word) in passphrase.iter().enumerate() {
        let idx = match all_words.iter().position(|w| *w == word.to_string()) {
            Some(x) => x,
            None => return false,
        };
        words[i] = idx;
        for j in 0..11 {
            bits[i * 11 + j] = idx >> (10 - j) & 1 == 1;
        }
    }
    let mut entropy = [0u8; 15 / 3 * 4];
    let nb_bytes_entropy = 15 / 3 * 4;
    for i in 0..nb_bytes_entropy {
        for j in 0..8 {
            if bits[i * 8 + j] {
                entropy[i] += 1 << (7 - j);
            }
        }
    }
    let check = sha256::Hash::hash(&entropy[0..nb_bytes_entropy]);
    for i in 0..nb_bytes_entropy / 4 {
        if bits[8 * nb_bytes_entropy + i] != ((check[i / 8] & (1 << (7 - (i % 8)))) > 0) {
            return false;
        }
    }
    true
}

fn main() {
    let language = Language::English;
    let all_words = language.words_by_prefix("");
    let words = "sock verb fiction spot repair cotton illness elbow olive core dove elevator van direct bronze";
    let mut count = 0;
    for word in all_words {
        let mut list: Vec<String> = words.split_whitespace().map(|s| s.into()).collect();
        let end = list.len() - 1;
        list[end] = word.to_string();
        if parse_15_words(all_words, list) {
            count += 1;
            println!("found: {}", word);
        }
    }
    println!("original passphrase: {}", words);
    println!("complete wordlist has {} words", all_words.len());
    println!("{} words possible as the last word", count);
}
