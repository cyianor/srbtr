use maplit::{convert_args, hashmap};
use std::collections::HashMap;
use std::io;
use std::io::{Bytes, Read};
use std::iter::Peekable;
use unicode_normalization::UnicodeNormalization;
use unicode_reader::{CodePoints, Graphemes};

/// Wraps a `String`-oriented reader and yields the data one cyrillic character at a time.
pub struct Transcoder<R: Iterator<Item = io::Result<String>>> {
    input: Peekable<R>,
    latin_to_cyrillic: HashMap<String, String>,
    composite_chars: HashMap<String, String>,
}

impl<R: Iterator<Item = io::Result<String>>> Iterator for Transcoder<R> {
    /// The type of the elements being iterated over: a `io::Result` with one
    /// cyrillic character, or any I/O error raised by the underlying reader
    type Item = io::Result<(String, String)>;

    /// Get the next cyrillic character from the stream.
    fn next(&mut self) -> Option<Self::Item> {
        let mut ch = match self.input.next() {
            Some(r) => match r {
                Ok(v) => v.nfc().collect::<String>(),
                Err(err) => return Some(Err(err)),
            },
            None => return None,
        };

        let mut orig = ch.clone();

        if self.composite_chars.contains_key(&ch) {
            ch = self.composite_chars[&ch].clone();
        } else if matches!(ch.as_str(), "D" | "d" | "L" | "l" | "N" | "n") {
            match self.input.peek() {
                Some(r) => {
                    // Lower-case on second letter to deal with full upper case
                    // of digraphs i.e. "LJ"
                    match r {
                        Ok(v) => {
                            let ch2 = v.clone().nfc().collect::<String>().to_lowercase();
                            let digraph = ch.clone() + &ch2;

                            // Officially, there is no letter "dj", but it is sometimes used
                            // instead of "đ"
                            if matches!(digraph.as_str(), "Dj" | "dj") {
                                match digraph.as_str() {
                                    "Dj" => ch = "Ð".to_string(),
                                    "dj" => ch = "đ".to_string(),
                                    _ => {}
                                }

                                orig = digraph.clone();
                                self.input.next();
                            }

                            if self.latin_to_cyrillic.contains_key(&digraph) {
                                ch.push_str(&ch2);
                                orig = digraph.clone();
                                self.input.next();
                            }
                        }
                        Err(_) => {}
                    };
                }
                None => {}
            };
        }

        if self.latin_to_cyrillic.contains_key(&ch) {
            return Some(Ok((orig, self.latin_to_cyrillic[&ch].clone())));
        } else {
            return Some(Ok((orig, ch)));
        }
    }
}

impl<R: Iterator<Item = io::Result<String>>> From<R> for Transcoder<R> {
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
