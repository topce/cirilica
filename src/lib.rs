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
            's' => output.push('с'),
            'o' => output.push('о'),
            'e' => output.push('е'),
            'a' => output.push('а'),
            'i' => output.push('и'),
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
            't' => output.push('т'),
            'r' => output.push('р'),
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
            'u' => output.push('у'),
            'k' => output.push('к'),
            'm' => output.push('м'),
            'b' => output.push('б'),
            'p' => output.push('п'),
            'z' => output.push('з'),
            'š' => output.push('ш'),
            'v' => output.push('в'),
            'j' => output.push('ј'),
            'h' => output.push('х'),
            'f' => output.push('ф'),
            'ž' => output.push('ж'),
            'g' => output.push('г'),
            'c' => output.push('ц'),
            'č' => output.push('ч'),
            'đ' => output.push('ђ'),
            'ć' => output.push('ћ'),
            'S' => output.push('С'),
            'O' => output.push('О'),
            'E' => output.push('Е'),
            'I' => output.push('И'),
            'A' => output.push('А'),
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
            'T' => output.push('Т'),
            'R' => output.push('Р'),
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
            'U' => output.push('У'),
            'K' => output.push('К'),
            'M' => output.push('М'),
            'B' => output.push('Б'),
            'P' => output.push('П'),
            'Z' => output.push('З'),
            'Š' => output.push('Ш'),
            'V' => output.push('В'),
            'J' => output.push('Ј'),
            'H' => output.push('Х'),
            'F' => output.push('Ф'),
            'Ž' => output.push('Ж'),
            'G' => output.push('Г'),
            'C' => output.push('Ц'),
            'Č' => output.push('Ч'),
            'Ć' => output.push('Ћ'),
            'Đ' => output.push('Ђ'),
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
