mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn to_serbian_cyrillic(word: String) -> String {
    let mut result = String::with_capacity(word.bytes().len() * 2);
    let mut chars = word.chars();
    let mut previous_char = '\0';
    loop {
        let letter = chars.next();

        match letter {
            Some(a) => match a {
                'n' | 'l' | 'd' | 'N' | 'L' | 'D' => {
                    if previous_char != '\0' {
                        result.push(to_serbian_cyrillic_char(previous_char));
                    }
                    previous_char = a;
                }

                'j' => match previous_char {
                    'l' => {
                        result.push('љ');
                        previous_char = '\0';
                    }
                    'L' => {
                        result.push('Љ');
                        previous_char = '\0';
                    }
                    'n' => {
                        result.push('њ');
                        previous_char = '\0';
                    }
                    'N' => {
                        result.push('Њ');
                        previous_char = '\0';
                    }
                    'd' | 'D' => {
                        result.push(to_serbian_cyrillic_char(previous_char));
                        result.push('ј');
                        previous_char = '\0';
                    }
                    '\0' => {
                        result.push('ј');
                    }
                    _ => {
                        result.push(to_serbian_cyrillic_char(previous_char));
                        result.push('ј');
                    }
                },
                'ž' => match previous_char {
                    'd' => {
                        result.push('џ');
                        previous_char = '\0';
                    }
                    'D' => {
                        result.push('Џ');
                        previous_char = '\0';
                    }
                    'l' | 'L' | 'n' | 'N' => {
                        result.push(to_serbian_cyrillic_char(previous_char));
                        result.push('ж');
                        previous_char = '\0';
                    }
                    '\0' => {
                        result.push('ж');
                    }
                    _ => {
                        result.push(to_serbian_cyrillic_char(previous_char));
                        result.push('ж');
                    }
                },

                _ => {
                    if previous_char != '\0' {
                        result.push(to_serbian_cyrillic_char(previous_char));
                        previous_char = '\0';
                    }
                    result.push(to_serbian_cyrillic_char(a));
                }
            },
            None => break,
        }
    }
    result.shrink_to_fit();

    result
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_azbuka_small() {
        assert_eq!(
            to_serbian_cyrillic("abvgdđežzijklljmnnjoprstćufhcčdžš".to_string()),
            "абвгдђежзијклљмнњопрстћуфхцчџш"
        );
    }

    #[test]
    fn test_azbuka_big() {
        assert_eq!(
            to_serbian_cyrillic("ABVGDĐEŽZIJKLLjMNNjOPRSTĆUFHCČDžŠ".to_string()),
            "АБВГДЂЕЖЗИЈКЛЉМНЊОПРСТЋУФХЦЧЏШ"
        );
    }
    #[test]
    fn test_azbuka_small_1000000() {
        assert_eq!(
            to_serbian_cyrillic(
                "abvgdđežzijklljmnnjoprstćufhcčdžš"
                    .repeat(1000000)
                    .to_string()
            ),
            "абвгдђежзијклљмнњопрстћуфхцчџш".repeat(1000000)
        );
    }

    #[test]
    fn test_azbuka_big_1000000() {
        assert_eq!(
            to_serbian_cyrillic(
                "ABVGDĐEŽZIJKLLjMNNjOPRSTĆUFHCČDžŠ"
                    .repeat(1000000)
                    .to_string()
            ),
            "АБВГДЂЕЖЗИЈКЛЉМНЊОПРСТЋУФХЦЧЏШ".repeat(1000000)
        );
    }
}
