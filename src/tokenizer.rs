// src/tokenizer.rs ~=#######D]====A===r===c====M===o===o===n====<Lord[TOKENIZER]Xyn>=====S===t===u====d===i===o===s====[R|$>
use std::collections::HashMap;

pub struct Tokenizer {
    vocab: HashMap<String, usize>,
    reverse_vocab: HashMap<usize, String>,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            vocab: HashMap::new(),
            reverse_vocab: HashMap::new(),
        }
    }

    pub fn decode(&self, tokens: &[i64]) -> String {
        tokens
            .iter()
            .map(|&token| self.reverse_vocab.get(&(token as usize)).unwrap_or(&"<UNK>".to_string()).clone())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn encode(&self, text: &str) -> Vec<i64> {
        text.split_whitespace()
            .map(|word| *self.vocab.get(word).unwrap_or(&self.vocab["<UNK>"]) as i64)
            .collect()
    }
}