#![allow(dead_code)]
/*
    probability distribution
        ↓
    selected TokenId
*/

// softmax

// Temperature:
// Изменяет степень случайности.
//
// Например:
//  T = 0.2
// модель становится более уверенной и детерминированной.
//  T = 1.0
// обычное распределение.
//  T = 2.0
// больше хаоса.
//
// Математически примерно:
// logits_i = logits_i / T

// random sampling

// top-k
// top-p
// min-p
// repetition penalty
