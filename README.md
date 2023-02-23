# WIP: Transliteration between the Serbian latin and cyrillic alphabets

A toy project on the transliteration between the Serbian latin and cyrillic
alphabets. Apart from achieving transliteration and practising Rust there is
currently no goal to this project.

Currently, the program takes a path to a text file containing Serbian latin
text and transliterates it to cyrillic. All characters that are not recognized
as latin characters are kept as-is. The program outputs both the latin and
cyrillic texts for easier comparison.

TODOs:
- Proper CLI
- Transliterate cyrillic -> latin (should be rather trivial)
- Make sure transliteration is idempotent given that text is properly
  unicode normalized
