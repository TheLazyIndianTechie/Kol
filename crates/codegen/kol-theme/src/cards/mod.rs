pub mod foundation;

/// Repo-relative directory the generated card library is written to.
pub const OUTPUT_DIR: &str = "design-system";

/// Every card the current phase can generate.
///
/// Foundation cards derive from tokens alone and ship in P1. Component cards (Field, Chrome,
/// States, Consent) require the components to exist and are added by the phase that builds
/// them — see the spec's §4.4 table.
pub fn all() -> Vec<Card> {
    vec![
        foundation::color_card(),
        foundation::type_card(),
        foundation::motion_card(),
        foundation::geometry_card(),
    ]
}

/// One design-system card: a standalone HTML file in the synced project.
///
/// Cards are entirely self-contained — the Design System pane blocks requests to external
/// hosts, so fonts, styles, and any assets must be inline. Kol's typeface is JetBrains Mono
/// with a monospace fallback chain rather than a web-font link, for the same reason.
pub struct Card {
    /// Card title, e.g. `Surfaces`.
    pub name: &'static str,
    /// Pane group this card files under, e.g. `Color`.
    pub group: &'static str,
    /// One-line summary shown under the title.
    pub subtitle: String,
    /// Card content as HTML.
    pub body: String,
}

impl Card {
    /// Render to a standalone HTML document.
    ///
    /// The first line is the `@dsCard` marker the Design System pane parses to build its card
    /// index — it must stay first and must stay on its own line.
    pub fn to_html(&self) -> String {
        format!(
            r#"<!-- @dsCard group="{group}" -->
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Kol · {name}</title>
<style>
  * {{ box-sizing: border-box; }}
  body {{
    margin: 0;
    padding: 28px 32px;
    background: {root};
    color: {body_fg};
    font-family: 'JetBrains Mono', ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
    font-size: 12.5px;
    line-height: 1.5;
    letter-spacing: .01em;
  }}
  header {{ margin-bottom: 26px; }}
  h1 {{
    margin: 0;
    font-size: 11.5px;
    font-weight: 700;
    letter-spacing: .24em;
    text-transform: uppercase;
    color: {primary};
  }}
  header p {{ margin: 7px 0 0; font-size: 10.5px; color: {dim}; letter-spacing: .08em; }}
  table {{ border-collapse: collapse; width: 100%; max-width: 860px; }}
  th {{
    text-align: left;
    font-size: 10px;
    font-weight: 400;
    letter-spacing: .18em;
    text-transform: uppercase;
    color: {dim};
    padding: 0 14px 9px 0;
    border-bottom: 1px solid {hairline};
  }}
  td {{ padding: 9px 14px 9px 0; border-bottom: 1px solid {hairline}; vertical-align: middle; }}
  .swatch {{ width: 34px; height: 18px; display: block; border: 1px solid {border_dim}; }}
  .name {{ color: {primary}; }}
  .hex {{ color: {muted}; }}
  .note {{ color: {dim}; }}
  .wrap {{ overflow-x: auto; }}
  section {{ margin-bottom: 34px; }}
  section h2 {{
    margin: 0 0 12px;
    font-size: 10px;
    font-weight: 400;
    letter-spacing: .2em;
    text-transform: uppercase;
    color: {dim};
  }}
</style>
</head>
<body>
<header>
<h1>{name}</h1>
<p>{subtitle}</p>
</header>
<main>
{content}
</main>
</body>
</html>
"#,
            group = self.group,
            name = self.name,
            subtitle = self.subtitle,
            content = self.body,
            root = hex_of(crate::color::surface::ROOT),
            body_fg = hex_of(crate::color::text::BODY),
            primary = hex_of(crate::color::text::PRIMARY),
            muted = hex_of(crate::color::text::MUTED),
            dim = hex_of(crate::color::text::DIM),
            hairline = hex_of(crate::color::surface::HAIRLINE),
            border_dim = hex_of(crate::color::surface::BORDER_DIM),
        )
    }

    /// Project-relative path this card writes to.
    pub fn path(&self) -> String {
        format!("{}.html", slug(self.name))
    }
}

/// Lowercase a name and replace anything that is not alphanumeric with a hyphen.
pub fn slug(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}

/// Render a [`ratatui::style::Color::Rgb`] back to a CSS hex string.
///
/// The card chrome styles itself from the same tokens the TUI uses, so the design system is
/// rendered in its own palette rather than a hand-written approximation of it.
pub fn hex_of(c: ratatui::style::Color) -> String {
    match c {
        ratatui::style::Color::Rgb(r, g, b) => format!("#{r:02x}{g:02x}{b:02x}"),
        other => panic!("hex_of expects Color::Rgb, got {other:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Card {
        Card {
            name: "Surfaces",
            group: "Color",
            subtitle: "8 tokens".to_string(),
            body: "<p>swatches here</p>".to_string(),
        }
    }

    #[test]
    fn first_line_is_the_ds_card_marker() {
        let html = sample().to_html();
        let first = html.lines().next().expect("non-empty output");
        assert_eq!(first, r#"<!-- @dsCard group="Color" -->"#);
    }

    #[test]
    fn includes_the_name_and_subtitle() {
        let html = sample().to_html();
        assert!(html.contains("Surfaces"));
        assert!(html.contains("8 tokens"));
    }

    #[test]
    fn includes_the_body_verbatim() {
        let html = sample().to_html();
        assert!(html.contains("<p>swatches here</p>"));
    }

    #[test]
    fn is_a_complete_document() {
        let html = sample().to_html();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.trim_end().ends_with("</html>"));
    }

    #[test]
    fn carries_no_external_references() {
        // The Design System pane blocks external hosts; everything must be inline.
        let html = sample().to_html();
        assert!(!html.contains("http://"), "external reference in card");
        assert!(!html.contains("https://"), "external reference in card");
    }

    #[test]
    fn slugs_are_filename_safe() {
        assert_eq!(slug("Surfaces"), "surfaces");
        assert_eq!(slug("Field depth"), "field-depth");
        assert_eq!(slug("Text ramp"), "text-ramp");
    }

    #[test]
    fn path_is_the_slugged_name() {
        assert_eq!(sample().path(), "surfaces.html");
    }
}
