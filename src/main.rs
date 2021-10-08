use bip39::*;
use bitcoin_hashes::{sha256, Hash};

// simplified copy of Mnemonic::parse_in, because it doesn't allow 15 word seeds
pub fn parse_15_words<'a>(passphrase: &[usize]) -> bool {
    assert_eq!(passphrase.len(), 15);
    let mut bits = [false; 15 * 11];
    for (i, word_index) in passphrase.iter().enumerate() {
        for j in 0..11 {
            bits[i * 11 + j] = word_index >> (10 - j) & 1 == 1;
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

fn word_to_index(all_words: &[&str], word: &str) -> usize {
    match all_words.iter().position(|w| *w == word) {
        Some(x) => x,
        None => return 0,
    }
}

fn index_to_word<'a>(all_words: &[&'a str], index: usize) -> &'a str {
    all_words[index]
}

fn word_list_to_index_list(all_words: &[&str], word_list: &[&str]) -> Vec<usize> {
    let mut index_list: Vec<usize> = Vec::new();
    for word in word_list {
        index_list.push(word_to_index(all_words, word));
    }
    index_list
}

fn main() {
    let language = Language::English;
    let all_words = language.words_by_prefix("");
    let words = "sock verb fiction spot repair cotton illness elbow olive core dove elevator van direct bronze";
    let all_words_index = word_list_to_index_list(all_words, all_words);
    let mut word_list_index = word_list_to_index_list(all_words, &words.split_whitespace().collect::<Vec<&str>>());
    let mut count = 0;
    for word_index in all_words_index {
        let end = word_list_index.len() - 1;
        word_list_index[end] = word_index;
        if parse_15_words(&word_list_index) {
            count += 1;
            println!("found: {}", index_to_word(all_words, word_index));
        }
    }
    println!("original passphrase: {}", words);
    println!("complete wordlist has {} words", all_words.len());
    println!("{} words possible as the last word", count);
}
