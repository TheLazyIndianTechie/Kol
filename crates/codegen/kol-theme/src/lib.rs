//! Kol design tokens.
//!
//! This crate is the single source of truth for Kol's visual language. The TUI reads these
//! constants directly; the `gen_cards` binary emits the design-system HTML from the same
//! declarations, so the two cannot drift.
//!
//! A terminal cannot express everything the HTML design does. Colour crosses exactly; the
//! design's 11-step font-size scale collapses to five emphasis roles ([`typography::Role`]);
//! letter-spacing survives as literal interleaved spaces ([`tracking::tracked`]).

pub mod cards;
pub mod color;
pub mod geometry;

mod hex;

pub mod motion;
pub mod tracking;
pub mod typography;

pub use hex::{composite, hex};
