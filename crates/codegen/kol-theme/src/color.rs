use ratatui::style::Color;

/// One named colour token: the value the TUI uses plus the metadata the card generator needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Swatch {
    /// Const name, e.g. `ACCENT`.
    pub name: &'static str,
    /// The `#rrggbb` literal as written in the design.
    pub hex: &'static str,
    /// The parsed value the TUI uses.
    pub color: Color,
    /// Where and why this colour is used.
    pub note: &'static str,
}

/// Declare a colour group once, producing the consts and the registry together.
///
/// This is the mechanism that makes drift impossible: there is no way to add a token to the
/// TUI without it appearing on the card, or vice versa.
macro_rules! palette {
    ($modname:ident, $title:literal, { $( $name:ident = $hexlit:literal, $note:literal; )+ }) => {
        pub mod $modname {
            use super::Swatch;
            use ratatui::style::Color;

            $( pub const $name: Color = crate::hex($hexlit); )+

            /// Human-readable group title, used by the card generator.
            pub const TITLE: &str = $title;

            /// Every token in this group, in declaration order.
            pub const SWATCHES: &[Swatch] = &[
                $( Swatch { name: stringify!($name), hex: $hexlit, color: $name, note: $note } ),+
            ];
        }
    };
}

palette!(surface, "Surfaces", {
    ROOT       = "#06070a", "Application background";
    PANEL      = "#080a0e", "Header, footer, grid cell background";
    RAIL       = "#07090c", "Left field rail";
    RAISED     = "#0b0f14", "Active row, selected mode card";
    BAR        = "#080b0f", "Screen header and action bar";
    MODAL      = "#0a0d12", "Consent dialog body";
    HAIRLINE   = "#161b22", "1px rule, grid gap";
    BORDER_DIM = "#1b2027", "Inactive control border";
});

palette!(text, "Text ramp", {
    PRIMARY = "#e8edf1", "Headings and Kol's own speech";
    BODY    = "#c3c9cf", "Default body text";
    MUTED   = "#8a949c", "Secondary detail, tool names";
    DIM     = "#566069", "Section labels, hints";
    FAINT   = "#3d454c", "Timings, counts";
    GHOST   = "#2f363d", "Diff gutter line numbers";
});

palette!(semantic, "Semantic", {
    ACCENT = "#5fd3e3", "Kol's voice, active state, focus";
    WARN   = "#e0a95c", "Uncommitted work, needs consent";
    OK     = "#74cfa4", "Success, apply affordance";
    ALERT  = "#e0674f", "Recording, destructive, reject";
    VIOLET = "#c98ce0", "Reserved: alternate field accent";
});

palette!(field, "Field depth", {
    NEAR       = "#5fd3e3", "Front layer, rail field";
    MID        = "#3f7f8a", "Middle layer, rail field";
    FAR        = "#2a3138", "Back layer, rail field";
    GRAPH_NEAR = "#5fd3e3", "Front layer, graph field";
    GRAPH_MID  = "#3a6f79", "Middle layer, graph field";
    GRAPH_FAR  = "#242a30", "Back layer, graph field";
});

palette!(diff, "Diff", {
    ADD_FG     = "#a8dfc6", "Added line text";
    ADD_GUTTER = "#2f6b53", "Added line number";
    ADD_BG     = "#0f1917", "Added row fill, rgba(116,207,164,.09) flattened over ROOT";
    DEL_FG     = "#e0a396", "Removed line text";
    DEL_GUTTER = "#7a3a30", "Removed line number";
    DEL_BG     = "#190f10", "Removed row fill, rgba(224,103,79,.09) flattened over ROOT";
});

/// Source colour of the added-row tint before compositing: `rgba(116,207,164,…)`.
pub const ADD_TINT_SOURCE: Color = crate::hex("#74cfa4");
/// Source colour of the removed-row tint before compositing: `rgba(224,103,79,…)`.
pub const DEL_TINT_SOURCE: Color = crate::hex("#e0674f");
/// The design's `.09` alpha expressed in 0–255.
pub const TINT_ALPHA: u8 = 23;

/// Every colour group, in card order.
pub const GROUPS: &[(&str, &[Swatch])] = &[
    (surface::TITLE, surface::SWATCHES),
    (text::TITLE, text::SWATCHES),
    (semantic::TITLE, semantic::SWATCHES),
    (field::TITLE, field::SWATCHES),
    (diff::TITLE, diff::SWATCHES),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_root_matches_the_design() {
        assert_eq!(surface::ROOT, crate::hex("#06070a"));
    }

    #[test]
    fn accent_matches_the_design() {
        assert_eq!(semantic::ACCENT, crate::hex("#5fd3e3"));
    }

    #[test]
    fn every_group_is_non_empty() {
        for (title, swatches) in GROUPS {
            assert!(!swatches.is_empty(), "group {title} has no swatches");
        }
    }

    #[test]
    fn swatch_hex_matches_its_color() {
        for (title, swatches) in GROUPS {
            for s in *swatches {
                assert_eq!(
                    crate::hex(s.hex),
                    s.color,
                    "{title}/{}: hex string and Color disagree",
                    s.name
                );
            }
        }
    }

    #[test]
    fn every_swatch_has_a_note() {
        for (_, swatches) in GROUPS {
            for s in *swatches {
                assert!(!s.note.is_empty(), "swatch {} has no note", s.name);
            }
        }
    }

    #[test]
    fn diff_fills_are_composited_from_the_design_alpha() {
        assert_eq!(
            diff::ADD_BG,
            crate::composite(ADD_TINT_SOURCE, surface::ROOT, TINT_ALPHA)
        );
        assert_eq!(
            diff::DEL_BG,
            crate::composite(DEL_TINT_SOURCE, surface::ROOT, TINT_ALPHA)
        );
    }
}
