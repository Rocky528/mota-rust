mod types;
pub mod english;

use types::CharFrequency;

pub fn get_char_frequency(v: &[u8]) -> CharFrequency {
    let mut result = [0.0; 256];

    for c in v {
        result[*c as usize] += 1.0;
    }

    for c in 0..=255 {
        result[c as usize] /= v.len() as f64;
    }

    result
}

pub fn compare_frequencies(cf1: &CharFrequency, cf2: &CharFrequency) -> f64 {
    let mut result = 0.0;

    for c in 0..=255 {
        let f1 = cf1[c];
        let f2 = cf2[c];

        result += (f1 - f2).abs()
    }

    result
}
