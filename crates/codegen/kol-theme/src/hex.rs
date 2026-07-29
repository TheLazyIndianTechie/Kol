use ratatui::style::Color;

/// Parse a `#rrggbb` literal into a [`Color`] at compile time.
///
/// Panics during const evaluation on a malformed literal, so a typo in any token declaration
/// fails the build instead of silently rendering the wrong colour.
pub const fn hex(s: &str) -> Color {
    let b = s.as_bytes();
    assert!(b.len() == 7, "hex literal must be exactly 7 bytes: #rrggbb");
    assert!(b[0] == b'#', "hex literal must start with '#'");
    Color::Rgb(
        nibble(b[1]) * 16 + nibble(b[2]),
        nibble(b[3]) * 16 + nibble(b[4]),
        nibble(b[5]) * 16 + nibble(b[6]),
    )
}

const fn nibble(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => panic!("hex literal contains a non-hex digit"),
    }
}

/// Composite `fg` over an opaque `bg` at `alpha` (0–255), matching CSS `rgba()` over a solid
/// backdrop.
///
/// A terminal cell has no alpha channel, so the design's translucent diff-row fills must be
/// pre-composited against the surface they sit on. Integer math throughout, so this is usable
/// in const context and its result is exactly reproducible.
pub const fn composite(fg: Color, bg: Color, alpha: u8) -> Color {
    let (fr, fg_g, fb) = match fg {
        Color::Rgb(r, g, b) => (r, g, b),
        _ => panic!("composite requires Color::Rgb"),
    };
    let (br, bg_g, bb) = match bg {
        Color::Rgb(r, g, b) => (r, g, b),
        _ => panic!("composite requires Color::Rgb"),
    };
    Color::Rgb(mix(fr, br, alpha), mix(fg_g, bg_g, alpha), mix(fb, bb, alpha))
}

const fn mix(f: u8, b: u8, a: u8) -> u8 {
    (((f as u16) * (a as u16) + (b as u16) * (255 - a as u16)) / 255) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Color;

    #[test]
    fn parses_the_root_surface() {
        assert_eq!(hex("#06070a"), Color::Rgb(0x06, 0x07, 0x0a));
    }

    #[test]
    fn parses_the_accent() {
        assert_eq!(hex("#5fd3e3"), Color::Rgb(0x5f, 0xd3, 0xe3));
    }

    #[test]
    fn parses_uppercase_digits() {
        assert_eq!(hex("#5FD3E3"), Color::Rgb(0x5f, 0xd3, 0xe3));
    }

    #[test]
    fn usable_in_const_context() {
        const OK: Color = hex("#74cfa4");
        assert_eq!(OK, Color::Rgb(0x74, 0xcf, 0xa4));
    }

    #[test]
    fn composites_the_added_row_fill() {
        // rgba(116,207,164,.09) over ROOT #06070a, alpha 0.09*255 ≈ 23
        assert_eq!(
            composite(hex("#74cfa4"), hex("#06070a"), 23),
            hex("#0f1917")
        );
    }

    #[test]
    fn composites_the_removed_row_fill() {
        // rgba(224,103,79,.09) over ROOT #06070a
        assert_eq!(
            composite(hex("#e0674f"), hex("#06070a"), 23),
            hex("#190f10")
        );
    }

    #[test]
    fn composite_at_zero_alpha_is_the_backdrop() {
        assert_eq!(composite(hex("#ffffff"), hex("#06070a"), 0), hex("#06070a"));
    }
}
