use super::{hex_of, Card};
use crate::{color, geometry, motion, tracking, typography};
use std::fmt::Write as _;

/// Every colour token, grouped as declared.
pub fn color_card() -> Card {
    let total: usize = color::GROUPS.iter().map(|(_, s)| s.len()).sum();
    let mut body = String::new();
    for (title, swatches) in color::GROUPS {
        let _ = write!(body, "<section><h2>{title}</h2><div class=\"wrap\"><table>");
        let _ = write!(
            body,
            "<tr><th></th><th>Token</th><th>Hex</th><th>Usage</th></tr>"
        );
        for s in *swatches {
            let _ = write!(
                body,
                "<tr><td><span class=\"swatch\" style=\"background:{hex}\"></span></td>\
                 <td class=\"name\">{name}</td><td class=\"hex\">{hex}</td>\
                 <td class=\"note\">{note}</td></tr>",
                hex = s.hex,
                name = s.name,
                note = s.note,
            );
        }
        body.push_str("</table></div></section>");
    }
    Card {
        name: "Color",
        group: "Color",
        subtitle: format!("{total} tokens across {} groups", color::GROUPS.len()),
        body,
    }
}

/// The five emphasis roles, each rendered in its own style, with the design measurement it
/// stands in for and whether tracking applies.
pub fn type_card() -> Card {
    let mut body = String::from(
        "<section><h2>Emphasis roles</h2><div class=\"wrap\"><table>\
         <tr><th>Role</th><th>Sample</th><th>Design source</th><th>Tracked</th></tr>",
    );
    for r in typography::ROLES {
        let sample = if r.is_tracked() {
            tracking::tracked("STANDING BY")
        } else {
            "The socket was closing on silence".to_string()
        };
        let fg = hex_of(r.style().fg.expect("every role sets a foreground"));
        let weight = if matches!(r, typography::Role::Display | typography::Role::Title) {
            "700"
        } else {
            "400"
        };
        let _ = write!(
            body,
            "<tr><td class=\"name\">{name}</td>\
             <td style=\"color:{fg};font-weight:{weight}\">{sample}</td>\
             <td class=\"note\">{source}</td><td class=\"note\">{tracked}</td></tr>",
            name = r.name(),
            source = r.design_source(),
            tracked = if r.is_tracked() { "yes" } else { "no" },
        );
    }
    body.push_str("</table></div></section>");

    let _ = write!(
        body,
        "<section><h2>Tracking</h2>\
         <p class=\"note\">A terminal cannot set sub-character spacing, so the design's \
         letter-spacing becomes literal interleaved spaces. Words are separated by two.</p>\
         <div class=\"wrap\"><table>\
         <tr><th>Input</th><th>Rendered</th></tr>\
         <tr><td class=\"hex\">KOL</td><td class=\"name\">{a}</td></tr>\
         <tr><td class=\"hex\">STANDING BY</td><td class=\"name\">{b}</td></tr>\
         <tr><td class=\"hex\">CONSENT REQUIRED</td><td class=\"name\">{c}</td></tr>\
         </table></div></section>",
        a = tracking::tracked("KOL"),
        b = tracking::tracked("STANDING BY"),
        c = tracking::tracked("CONSENT REQUIRED"),
    );

    Card {
        name: "Type",
        group: "Type",
        subtitle: format!("{} emphasis roles, tracking", typography::ROLES.len()),
        body,
    }
}

/// Timing constants with the CSS rule each was transcribed from.
pub fn motion_card() -> Card {
    let mut body = String::from(
        "<section><h2>Timings</h2><div class=\"wrap\"><table>\
         <tr><th>Token</th><th>Duration</th><th>Design source</th><th>Animates</th></tr>",
    );
    for t in motion::TIMINGS {
        let _ = write!(
            body,
            "<tr><td class=\"name\">{name}</td><td class=\"hex\">{ms} ms</td>\
             <td class=\"note\">{source}</td><td class=\"note\">{note}</td></tr>",
            name = t.name,
            ms = t.duration.as_millis(),
            source = t.source,
            note = t.note,
        );
    }
    body.push_str("</table></div></section>");
    Card {
        name: "Motion",
        group: "Motion",
        subtitle: format!("{} timing tokens", motion::TIMINGS.len()),
        body,
    }
}

/// Field dimensions and the braille sub-pixel math they imply.
pub fn geometry_card() -> Card {
    let rail = geometry::FieldGeometry::RAIL;
    let graph = geometry::FieldGeometry::GRAPH;
    let (rw, rh) = rail.subpixels();
    let (gw, gh) = graph.subpixels();
    let body = format!(
        "<section><h2>Field surfaces</h2><div class=\"wrap\"><table>\
         <tr><th>Surface</th><th>Cells</th><th>Sub-pixels</th><th>Capacity</th></tr>\
         <tr><td class=\"name\">RAIL</td><td class=\"hex\">{rc} x {rr}</td>\
         <td class=\"hex\">{rw} x {rh}</td><td class=\"note\">{rcap}</td></tr>\
         <tr><td class=\"name\">GRAPH</td><td class=\"hex\">{gc} x {gr}</td>\
         <td class=\"hex\">{gw} x {gh}</td><td class=\"note\">{gcap}</td></tr>\
         </table></div></section>\
         <section><h2>Braille cell</h2>\
         <p class=\"note\">Base codepoint U+{base:04X}. Each cell packs a {cc} x {cr} \
         sub-pixel grid, giving the field four times the terminal's vertical resolution.</p>\
         </section>\
         <section><h2>Density</h2><div class=\"wrap\"><table>\
         <tr><th>Token</th><th>Value</th></tr>\
         <tr><td class=\"name\">DENSITY_DEFAULT</td><td class=\"hex\">{dd}</td></tr>\
         <tr><td class=\"name\">DENSITY_MIN</td><td class=\"hex\">{dmin}</td></tr>\
         <tr><td class=\"name\">DENSITY_MAX</td><td class=\"hex\">{dmax}</td></tr>\
         </table></div></section>",
        rc = rail.cols,
        rr = rail.rows,
        rcap = rail.capacity(),
        gc = graph.cols,
        gr = graph.rows,
        gcap = graph.capacity(),
        base = geometry::BRAILLE_BASE,
        cc = geometry::CELL_COLS,
        cr = geometry::CELL_ROWS,
        dd = geometry::DENSITY_DEFAULT,
        dmin = geometry::DENSITY_MIN,
        dmax = geometry::DENSITY_MAX,
    );
    Card {
        name: "Geometry",
        group: "Geometry",
        subtitle: "Field surfaces, braille cell, density range".to_string(),
        body,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colour_card_lists_every_token() {
        let card = color_card();
        let total: usize = crate::color::GROUPS.iter().map(|(_, s)| s.len()).sum();
        for (_, swatches) in crate::color::GROUPS {
            for s in *swatches {
                assert!(card.body.contains(s.hex), "{} missing from colour card", s.name);
                assert!(card.body.contains(s.name), "{} name missing", s.name);
            }
        }
        assert!(total >= 30, "expected the full palette, found {total}");
    }

    #[test]
    fn type_card_shows_tracking_applied() {
        let card = type_card();
        // Display and Label are tracked, so the card must show the spaced form.
        assert!(card.body.contains("S T A N D I N G"), "tracked sample missing");
    }

    #[test]
    fn type_card_lists_every_role() {
        let card = type_card();
        for r in crate::typography::ROLES {
            assert!(card.body.contains(r.name()), "{} missing from type card", r.name());
            assert!(card.body.contains(r.design_source()), "{} source missing", r.name());
        }
    }

    #[test]
    fn motion_card_lists_every_timing() {
        let card = motion_card();
        for t in crate::motion::TIMINGS {
            assert!(card.body.contains(t.name), "{} missing from motion card", t.name);
            assert!(card.body.contains(t.source), "{} source missing", t.name);
        }
    }

    #[test]
    fn geometry_card_states_both_fields() {
        let card = geometry_card();
        assert!(card.body.contains("46"), "rail cols missing");
        assert!(card.body.contains("108"), "graph cols missing");
        assert!(card.body.contains("2300"), "default density missing");
    }

    #[test]
    fn all_cards_have_distinct_paths() {
        let cards = crate::cards::all();
        let mut paths: Vec<String> = cards.iter().map(|c| c.path()).collect();
        paths.sort();
        let before = paths.len();
        paths.dedup();
        assert_eq!(before, paths.len(), "two cards write to the same path");
    }

    #[test]
    fn all_returns_the_four_foundation_cards() {
        assert_eq!(crate::cards::all().len(), 4);
    }
}
