//! Generate the Kol design-system card library from the tokens in `kol-theme`.
//!
//! Run from anywhere in the repo:
//!
//! ```text
//! cargo run -p kol-theme --bin gen_cards
//! ```
//!
//! Output is written to `design-system/` at the repo root and is committed. The
//! `no_drift` integration test fails if the committed files stop matching what the tokens
//! produce, which is what keeps the design system from drifting away from the binary.

use kol_theme::cards;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let out = output_dir();
    std::fs::create_dir_all(&out)?;

    let cards = cards::all();
    for card in &cards {
        let path = out.join(card.path());
        std::fs::write(&path, card.to_html())?;
        println!("wrote {}", path.display());
    }
    println!("{} cards generated", cards.len());
    Ok(())
}

/// Repo-root `design-system/`, resolved from this crate's manifest directory so the binary
/// works regardless of the shell's current directory.
fn output_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join(cards::OUTPUT_DIR)
}
