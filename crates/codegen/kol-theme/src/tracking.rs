/// Emulate CSS `letter-spacing` by interleaving spaces between characters.
///
/// The design applies `.1em`–`.28em` tracking to every label and display string (`KOL`,
/// `STANDING BY`, `RECORDING`, `CONSENT REQUIRED`). A terminal cannot space glyphs
/// sub-character, so the equivalent is a literal space between them. Words are separated by
/// two spaces so they remain distinguishable from intra-word spacing.
///
/// Leading and trailing whitespace is dropped and internal runs collapse to a single word
/// break, which matches how these strings are used — they are short fixed labels, never
/// user-supplied prose.
///
/// ```
/// use kol_theme::tracking::tracked;
/// assert_eq!(tracked("STANDING BY"), "S T A N D I N G  B Y");
/// ```
pub fn tracked(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut out = String::with_capacity(word.chars().count() * 2);
            for (i, c) in word.chars().enumerate() {
                if i > 0 {
                    out.push(' ');
                }
                out.push(c);
            }
            out
        })
        .collect::<Vec<_>>()
        .join("  ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracks_a_single_word() {
        assert_eq!(tracked("KOL"), "K O L");
    }

    #[test]
    fn separates_words_with_two_spaces() {
        assert_eq!(tracked("STANDING BY"), "S T A N D I N G  B Y");
    }

    #[test]
    fn single_character_is_unchanged() {
        assert_eq!(tracked("A"), "A");
    }

    #[test]
    fn empty_input_is_empty() {
        assert_eq!(tracked(""), "");
    }

    #[test]
    fn collapses_runs_of_whitespace() {
        assert_eq!(tracked("  CONSENT   REQUIRED  "), "C O N S E N T  R E Q U I R E D");
    }

    #[test]
    fn handles_multibyte_characters() {
        assert_eq!(tracked("◈◆"), "◈ ◆");
    }
}
