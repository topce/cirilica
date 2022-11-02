mod utils;

use std::char;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn to_serbian_cyrillic(word: String) -> String {
    word.chars()
        .into_iter()
        .map(|x| to_serbian_cyrillic_char(x))
        .collect::<String>()
}

fn to_serbian_cyrillic_char(letter: char) -> char {
    match letter {
        'A' => 'А',
        'B' => 'Б',
        'C' => 'Ц',
        'D' => 'Д',
        'E' => 'Е',
        'F' => 'Ф',
        'G' => 'Г',
        'H' => 'Х',
        'I' => 'И',
        'J' => 'Ј',
        'K' => 'К',
        'L' => 'Л',
        'M' => 'М',
        'N' => 'Н',
        'O' => 'О',
        'P' => 'П',
        'R' => 'Р',
        'S' => 'С',
        'T' => 'Т',
        'U' => 'У',
        'V' => 'В',
        'Z' => 'З',
        'a' => 'а',
        'b' => 'б',
        'c' => 'ц',
        'd' => 'д',
        'e' => 'е',
        'f' => 'ф',
        'g' => 'г',
        'h' => 'х',
        'i' => 'и',
        'j' => 'ј',
        'k' => 'к',
        'l' => 'л',
        'm' => 'м',
        'n' => 'н',
        'o' => 'о',
        'p' => 'п',
        'r' => 'р',
        's' => 'с',
        't' => 'т',
        'u' => 'у',
        'v' => 'в',
        'z' => 'з',
        'Ć' => 'Ћ',
        'ć' => 'ћ',
        'Č' => 'Ч',
        'č' => 'ч',
        'Đ' => 'Ђ',
        'đ' => 'ђ',
        'Š' => 'Ш',
        'š' => 'ш',
        'Ž' => 'Ж',
        'ž' => 'ж',
        _ => letter,
    }
}
