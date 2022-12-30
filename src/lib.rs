mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
/// transform utf-8 serbian latinic to serbian cyrilic
/// also translate two letters nj,lj and dž
/// and capital Nj,Lj and Dž
/// in one cyrilic letter  
pub fn latin_to_cyrillic(input: String) -> String {
    let mut output = String::new();
    let mut iter = input.chars().peekable();

    while let Some(c) = iter.next() {
        match c {
            'a' => output.push('а'),
            'b' => output.push('б'),
            'c' => output.push('ц'),
            'č' => output.push('ч'),
            'ć' => output.push('ћ'),
            'd' => {
                if let Some(&next) = iter.peek() {
                    if next == 'ž' {
                        output.push('џ');
                        iter.next();
                    } else {
                        output.push('д');
                    }
                } else {
                    output.push('д');
                }
            }
            'đ' => output.push('ђ'),
            'e' => output.push('е'),
            'ž' => output.push('ж'),
            'f' => output.push('ф'),
            'g' => output.push('г'),
            'h' => output.push('х'),
            'i' => output.push('и'),
            'j' => output.push('ј'),
            'k' => output.push('к'),
            'l' => {
                if let Some(&next) = iter.peek() {
                    if next == 'j' {
                        output.push('љ');
                        iter.next();
                    } else {
                        output.push('л');
                    }
                } else {
                    output.push('л');
                }
            }
            'm' => output.push('м'),
            'n' => {
                if let Some(&next) = iter.peek() {
                    if next == 'j' {
                        output.push('њ');
                        iter.next();
                    } else {
                        output.push('н');
                    }
                } else {
                    output.push('н');
                }
            }
            'o' => output.push('о'),
            'p' => output.push('п'),
            'r' => output.push('р'),
            's' => output.push('с'),
            'š' => output.push('ш'),
            't' => output.push('т'),
            'u' => output.push('у'),
            'v' => output.push('в'),
            'z' => output.push('з'),
            'A' => output.push('А'),
            'B' => output.push('Б'),
            'V' => output.push('В'),
            'G' => output.push('Г'),
            'D' => {
                if let Some(&next) = iter.peek() {
                    if next == 'ž' {
                        output.push('Џ');
                        iter.next();
                    } else {
                        output.push('Д');
                    }
                } else {
                    output.push('Д');
                }
            }
            'Đ' => output.push('Ђ'),
            'E' => output.push('Е'),
            'Ž' => output.push('Ж'),
            'Z' => output.push('З'),
            'I' => output.push('И'),
            'J' => output.push('Ј'),
            'K' => output.push('К'),
            'L' => {
                if let Some(&next) = iter.peek() {
                    if next == 'j' {
                        output.push('Љ');
                        iter.next();
                    } else {
                        output.push('Л');
                    }
                } else {
                    output.push('Л');
                }
            }
            'M' => output.push('М'),
            'N' => {
                if let Some(&next) = iter.peek() {
                    if next == 'j' {
                        output.push('Њ');
                        iter.next();
                    } else {
                        output.push('Н');
                    }
                } else {
                    output.push('Н');
                }
            }
            'O' => output.push('О'),
            'P' => output.push('П'),
            'R' => output.push('Р'),
            'S' => output.push('С'),
            'T' => output.push('Т'),
            'Ć' => output.push('Ћ'),
            'U' => output.push('У'),
            'F' => output.push('Ф'),
            'H' => output.push('Х'),
            'C' => output.push('Ц'),
            'Č' => output.push('Ч'),
            'Š' => output.push('Ш'),
            _ => output.push(c),
        }
    }

    output
}
/// transform utf-8 serbian latinic to serbian cyrilic
/// does translate two letters nj,lj and dž
/// and capital Nj,Lj and Dž
/// in one cyrilic letter but
/// in 2 cyrilic letters  

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_azbuka_small() {
        assert_eq!(
            latin_to_cyrillic("abvgdđežzijklljmnnjoprstćufhcčdžš".to_string()),
            "абвгдђежзијклљмнњопрстћуфхцчџш"
        );
    }

    #[test]
    fn test_azbuka_big() {
        assert_eq!(
            latin_to_cyrillic("ABVGDĐEŽZIJKLLjMNNjOPRSTĆUFHCČDžŠ".to_string()),
            "АБВГДЂЕЖЗИЈКЛЉМНЊОПРСТЋУФХЦЧЏШ"
        );
    }
    #[test]
    fn test_azbuka_small_1000000() {
        assert_eq!(
            latin_to_cyrillic(
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
            latin_to_cyrillic(
                "ABVGDĐEŽZIJKLLjMNNjOPRSTĆUFHCČDžŠ"
                    .repeat(1000000)
                    .to_string()
            ),
            "АБВГДЂЕЖЗИЈКЛЉМНЊОПРСТЋУФХЦЧЏШ".repeat(1000000)
        );
    }
    #[test]
    fn test_njljdž() {
        assert_eq!(latin_to_cyrillic("njljdžNjLjDž".to_string()), "њљџЊЉЏ");
    }
}
