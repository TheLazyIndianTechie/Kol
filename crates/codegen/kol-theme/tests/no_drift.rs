//! Fails if the committed card library no longer matches what the tokens produce.
//!
//! This is the mechanism behind the spec's claim that the design system cannot drift from the
//! binary: regenerating is the only way to change a card, and forgetting to regenerate is a
//! test failure rather than a silent inconsistency.

use kol_theme::cards;
use std::path::PathBuf;

fn output_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join(cards::OUTPUT_DIR)
}

#[test]
fn committed_cards_match_the_tokens() {
    let dir = output_dir();
    for card in cards::all() {
        let path = dir.join(card.path());
        let committed = std::fs::read_to_string(&path).unwrap_or_else(|e| {
            panic!(
                "{} is missing ({e}). Run: cargo run -p kol-theme --bin gen_cards",
                path.display()
            )
        });
        assert_eq!(
            committed,
            card.to_html(),
            "{} is stale. Run: cargo run -p kol-theme --bin gen_cards",
            path.display()
        );
    }
}

#[test]
fn no_orphaned_cards() {
    let dir = output_dir();
    let expected: Vec<String> = cards::all().iter().map(|c| c.path()).collect();
    let entries = std::fs::read_dir(&dir).expect("design-system/ exists");
    for entry in entries {
        let name = entry.expect("readable entry").file_name();
        let name = name.to_string_lossy().to_string();
        if !name.ends_with(".html") {
            continue;
        }
        assert!(
            expected.contains(&name),
            "{name} is in design-system/ but no card generates it — delete it or add the builder"
        );
    }
}
