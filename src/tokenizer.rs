#![allow(dead_code)]

use std::collections::HashMap;

// Токен (минимальная единица, которой оперирует модель)
pub type TokenId = u32;

/// Список всех возможных токенов
pub struct Vocabulary {
    char_to_id: HashMap<char, TokenId>,
    id_to_char: Vec<char>,
}

// TokenId
//    ↓
// индекс vocabulary
//    ↓
//  char

// text -> tokens
pub fn encode(string: &str) -> Vec<u32> {
    string.chars().map(|c| c as u32).collect()
}

// tokens -> text
pub fn decode(tokens: &[u32]) -> String {
    
    tokens.iter().map(|&t| char::from_u32(t as u32).unwrap()).collect()
}
