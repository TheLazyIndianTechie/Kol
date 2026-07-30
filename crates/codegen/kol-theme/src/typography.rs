use crate::color;
use ratatui::style::{Modifier, Style};

/// The five emphasis roles the design's 11-step font-size scale collapses to.
///
/// A terminal has no font sizes. Hierarchy is carried by colour, weight, and tracking instead.
/// Each variant records the design measurement it stands in for, so the card generator can
/// show the original alongside the terminal equivalent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// The one oversized string on a screen. `STANDING BY`.
    Display,
    /// Screen titles and Kol's own speech.
    Title,
    /// Default running text.
    Body,
    /// Uppercase section labels. `TRANSCRIPT`, `BLAST RADIUS`.
    Label,
    /// Timings, counts, keyboard hints.
    Meta,
}

/// Every role, in hierarchy order.
pub const ROLES: &[Role] = &[
    Role::Display,
    Role::Title,
    Role::Body,
    Role::Label,
    Role::Meta,
];

impl Role {
    /// The ratatui style for this role.
    pub fn style(self) -> Style {
        match self {
            Role::Display => Style::default()
                .fg(color::semantic::ACCENT)
                .add_modifier(Modifier::BOLD),
            Role::Title => Style::default()
                .fg(color::text::PRIMARY)
                .add_modifier(Modifier::BOLD),
            Role::Body => Style::default().fg(color::text::BODY),
            Role::Label => Style::default().fg(color::text::DIM),
            Role::Meta => Style::default().fg(color::text::FAINT),
        }
    }

    /// Whether strings in this role should be passed through
    /// [`crate::tracking::tracked`] before rendering.
    pub const fn is_tracked(self) -> bool {
        matches!(self, Role::Display | Role::Label)
    }

    /// The design measurement this role stands in for.
    pub const fn design_source(self) -> &'static str {
        match self {
            Role::Display => "22px / letter-spacing .28em",
            Role::Title => "14–17px",
            Role::Body => "12.5px / line-height 1.7",
            Role::Label => "10–10.5px / letter-spacing .18–.24em",
            Role::Meta => "10.5px",
        }
    }

    /// Lowercase name used in generated card markup.
    pub const fn name(self) -> &'static str {
        match self {
            Role::Display => "Display",
            Role::Title => "Title",
            Role::Body => "Body",
            Role::Label => "Label",
            Role::Meta => "Meta",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color;

    #[test]
    fn display_is_accent_and_bold() {
        let s = Role::Display.style();
        assert_eq!(s.fg, Some(color::semantic::ACCENT));
        assert!(s.add_modifier.contains(ratatui::style::Modifier::BOLD));
    }

    #[test]
    fn body_carries_no_modifier() {
        let s = Role::Body.style();
        assert_eq!(s.fg, Some(color::text::BODY));
        assert_eq!(s.add_modifier, ratatui::style::Modifier::empty());
    }

    #[test]
    fn only_display_and_label_are_tracked() {
        assert!(Role::Display.is_tracked());
        assert!(Role::Label.is_tracked());
        assert!(!Role::Title.is_tracked());
        assert!(!Role::Body.is_tracked());
        assert!(!Role::Meta.is_tracked());
    }

    #[test]
    fn every_role_documents_its_design_source() {
        for r in ROLES {
            assert!(!r.design_source().is_empty(), "{r:?} has no design source");
        }
    }

    #[test]
    fn roles_list_is_complete() {
        assert_eq!(ROLES.len(), 5);
    }
}
