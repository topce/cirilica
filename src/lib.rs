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
    let utf_word = word
        .replace("nj", "ǌ")
        .replace("lj", "ǉ")
        .replace("Nj", "ǋ")
        .replace("Lj", "ǈ");
    utf_word
        .chars()
        .into_iter()
        .map(|x| to_serbian_cyrillic_char(x))
        .collect::<String>()
}

fn to_serbian_cyrillic_char(letter: char) -> char {
    match letter {
        'A' => 'А',
        'B' => 'Б',
        'V' => 'В',
        'G' => 'Г',
        'D' => 'Д',
        'Đ' => 'Ђ',
        'E' => 'Е',
        'Ž' => 'Ж',
        'Z' => 'З',
        'I' => 'И',
        'J' => 'Ј',
        'K' => 'К',
        'L' => 'Л',
        'ǈ' => 'Љ',
        'M' => 'М',
        'N' => 'Н',
        'ǋ' => 'Њ',
        'O' => 'О',
        'P' => 'П',
        'R' => 'Р',
        'S' => 'С',
        'T' => 'Т',
        'Ć' => 'Ћ',
        'U' => 'У',
        'F' => 'Ф',
        'H' => 'Х',
        'C' => 'Ц',
        'Č' => 'Ч',
        'ǅ' => 'Џ',
        'Š' => 'Ш',

        'a' => 'а',
        'b' => 'б',
        'v' => 'в',
        'g' => 'г',
        'd' => 'д',
        'đ' => 'ђ',
        'e' => 'е',
        'ž' => 'ж',
        'z' => 'з',
        'i' => 'и',
        'j' => 'ј',
        'k' => 'к',
        'l' => 'л',
        'ǉ' => 'љ',
        'm' => 'м',
        'n' => 'н',
        'ǌ' => 'њ',
        'o' => 'о',
        'p' => 'п',
        'r' => 'р',
        's' => 'с',
        't' => 'т',
        'ć' => 'ћ',
        'u' => 'у',
        'f' => 'ф',
        'h' => 'х',
        'c' => 'ц',
        'č' => 'ч',
        'ǆ' => 'џ',
        'š' => 'ш',
        _ => letter,
    }
}
