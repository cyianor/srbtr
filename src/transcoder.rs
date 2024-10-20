use maplit::{convert_args, hashmap};
use std::collections::HashMap;
use std::io;
use std::io::{Bytes, Read};
use std::iter::Peekable;
use unicode_normalization::UnicodeNormalization;
use unicode_reader::{CodePoints, Graphemes};

/// Wraps a `String`-oriented reader and yields the data one cyrillic character at a time.
pub struct Transcoder<R>
where
    R: Iterator<Item = io::Result<String>>,
{
    input: Peekable<R>,
    latin_to_cyrillic: HashMap<String, String>,
    composite_chars: HashMap<String, String>,
}

impl<R> Iterator for Transcoder<R>
where
    R: Iterator<Item = io::Result<String>>,
{
    /// The type of the elements being iterated over: a `io::Result` with
    /// tuples of input (latin) and output (cyrillic) characters,
    /// or any I/O error raised by the underlying reader
    type Item = io::Result<(String, String)>;

    /// Get the next character from the stream.
    fn next(&mut self) -> Option<Self::Item> {
        let latin = match self.input.next()? {
            Ok(v) => v.nfc().collect::<String>(),
            Err(err) => return Some(Err(err)),
        };

        let (original, modified) = if self.composite_chars.contains_key(&latin) {
            (latin.clone(), self.composite_chars[&latin].clone())
        } else if matches!(latin.as_str(), "D" | "d" | "L" | "l" | "N" | "n") {
            // Lower-case on second letter to deal with full upper case
            // of digraphs i.e. "LJ"
            if let Some(v) = self.input.peek() {
                match v {
                    Ok(next_grapheme) => {
                        let digraph =
                            latin.clone() + &next_grapheme.nfc().collect::<String>().to_lowercase();
                        // Officially, there is no letter "dj", but it is sometimes used
                        // instead of "đ"
                        if let Some(out) = match digraph.as_str() {
                            "Dj" => Some(("Dj".to_string(), "Ð".to_string())),
                            "dj" => Some(("dj".to_string(), "đ".to_string())),
                            _ => {
                                if self.latin_to_cyrillic.contains_key(&digraph) {
                                    Some((digraph.clone(), digraph))
                                } else {
                                    None
                                }
                            }
                        } {
                            self.input.next();
                            out
                        } else {
                            (latin.clone(), latin)
                        }
                    }
                    Err(_) => (latin.clone(), latin),
                }
            } else {
                (latin.clone(), latin)
            }
        } else {
            (latin.clone(), latin)
        };

        if self.latin_to_cyrillic.contains_key(&modified) {
            Some(Ok((original, self.latin_to_cyrillic[&modified].clone())))
        } else {
            Some(Ok((original, modified.clone())))
        }
    }
}

impl<R> From<R> for Transcoder<R>
where
    R: Iterator<Item = io::Result<String>>,
{
    fn from(input: R) -> Transcoder<R> {
        Transcoder {
            input: input.peekable(),
            latin_to_cyrillic: convert_args!(
                keys = |k: &str| k.nfc().collect::<String>(),
                values = |k: &str| k.nfc().collect::<String>(),
                hashmap!(
                    "A" => "А", "a" => "а",
                    "B" => "Б", "b" => "б",
                    "V" => "В", "v" => "в",
                    "G" => "Г", "g" => "г",
                    "D" => "Д", "d" => "д",
                    "Ð" => "Ђ", "đ" => "ђ",
                    "E" => "Е", "e" => "е",
                    "Ž" => "Ж", "ž" => "ж",
                    "Z" => "З", "z" => "з",
                    "I" => "И", "i" => "и",
                    "J" => "Ј", "j" => "ј",
                    "K" => "К", "k" => "к",
                    "L" => "Л", "l" => "л",
                    "Lj" => "Љ", "lj" => "љ",
                    "M" => "М", "m" => "м",
                    "N" => "Н", "n" => "н",
                    "Nj" => "Њ", "nj" => "њ",
                    "O" => "О", "o" => "о",
                    "P" => "П", "p" => "п",
                    "R" => "Р", "r" => "р",
                    "S" => "С", "s" => "с",
                    "T" => "Т", "t" => "т",
                    "Ć" => "Ћ", "ć" => "ћ",
                    "U" => "У", "u" => "у",
                    "F" => "Ф", "f" => "ф",
                    "H" => "Х", "h" => "х",
                    "C" => "Ц", "c" => "ц",
                    "Č" => "Ч", "č" => "ч",
                    "Dž" => "Џ", "dž" => "џ",
                    "Š" => "Ш", "š" => "ш",
                )
            ),
            composite_chars: convert_args!(
                keys = |k: &str| k.nfc().collect::<String>(),
                values = String::from,
                hashmap!(
                    "Ǆ" => "Dž",
                    "ǅ" => "Dž",
                    "ǆ" => "dž",
                    "Ǉ" => "Lj",
                    "ǈ" => "Lj",
                    "ǉ" => "lj",
                    "Ǌ" => "Nj",
                    "ǋ" => "Nj",
                    "ǌ" => "nj",
                )
            ),
        }
    }
}

impl<R: Read> From<R> for Transcoder<Graphemes<CodePoints<Bytes<R>>>> {
    fn from(input: R) -> Transcoder<Graphemes<CodePoints<Bytes<R>>>> {
        Transcoder::from(Graphemes::from(input))
    }
}
