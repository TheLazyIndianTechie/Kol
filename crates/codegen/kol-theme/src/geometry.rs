/// Base codepoint of the Unicode braille block. Each cell packs a 2×4 sub-pixel grid, which is
/// what lets the field render at four times the terminal's vertical resolution.
pub const BRAILLE_BASE: u32 = 0x2800;

/// Sub-pixel columns per braille cell.
pub const CELL_COLS: u16 = 2;

/// Sub-pixel rows per braille cell.
pub const CELL_ROWS: u16 = 4;

/// Point-cloud density: the prototype's `density` prop default.
pub const DENSITY_DEFAULT: usize = 2300;
/// Lowest density the prototype permits.
pub const DENSITY_MIN: usize = 300;
/// Highest density the prototype permits.
pub const DENSITY_MAX: usize = 2600;

// Compile-time invariants: the default density must lie inside the permitted range.
// Checked on every build, not only under `cargo test`.
const _: () = assert!(DENSITY_MIN <= DENSITY_DEFAULT);
const _: () = assert!(DENSITY_DEFAULT <= DENSITY_MAX);

/// Render geometry of a field surface, measured in terminal cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldGeometry {
    pub cols: u16,
    pub rows: u16,
}

impl FieldGeometry {
    /// The left rail field. 46×32 cells in the prototype.
    pub const RAIL: Self = Self { cols: 46, rows: 32 };

    /// The full-width graph field. 108×32 cells in the prototype.
    pub const GRAPH: Self = Self { cols: 108, rows: 32 };

    /// Sub-pixel resolution of this field, as `(width, height)`.
    pub const fn subpixels(self) -> (u16, u16) {
        (self.cols * CELL_COLS, self.rows * CELL_ROWS)
    }

    /// Total addressable sub-pixels. An upper bound on useful point density.
    pub const fn capacity(self) -> u32 {
        let (w, h) = self.subpixels();
        w as u32 * h as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rail_subpixel_resolution() {
        // 46 cells wide × 2 sub-columns, 32 cells tall × 4 sub-rows
        assert_eq!(FieldGeometry::RAIL.subpixels(), (92, 128));
    }

    #[test]
    fn graph_subpixel_resolution() {
        assert_eq!(FieldGeometry::GRAPH.subpixels(), (216, 128));
    }

    #[test]
    fn rail_capacity() {
        assert_eq!(FieldGeometry::RAIL.capacity(), 92 * 128);
    }

    #[test]
    fn default_density_fits_the_rail_field() {
        assert!(
            DENSITY_DEFAULT < FieldGeometry::RAIL.capacity() as usize,
            "more points than addressable sub-pixels"
        );
    }
}
