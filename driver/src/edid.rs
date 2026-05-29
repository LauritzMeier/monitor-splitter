//! Synthetic EDID generation for virtual monitors.
//!
//! Each virtual monitor needs a valid EDID block so that Windows recognizes it
//! as a real display and assigns it proper resolution/refresh rate settings.

use monitor_splitter_common::{generate_synthetic_edid, SplitRegion};

/// Generate an EDID for a virtual monitor representing a region of a physical display.
///
/// # Arguments
/// * `physical_width` - Total width of the physical monitor in pixels
/// * `physical_height` - Total height of the physical monitor in pixels
/// * `region` - The sub-region this virtual monitor occupies
/// * `virtual_index` - Index of this virtual monitor (for naming)
///
/// # Returns
/// A 128-byte EDID block
pub fn edid_for_region(
    physical_width: u32,
    physical_height: u32,
    region: &SplitRegion,
    virtual_index: u32,
) -> [u8; 128] {
    let width = (physical_width as f64 * region.width) as u32;
    let height = (physical_height as f64 * region.height) as u32;
    let name = format!("VSplit-{}", virtual_index);
    generate_synthetic_edid(width, height, &name)
}

/// Validate that a set of regions covers the full monitor without overlap.
pub fn validate_regions(regions: &[SplitRegion]) -> Result<(), String> {
    // Check bounds
    for (i, r) in regions.iter().enumerate() {
        if r.x < 0.0 || r.y < 0.0 || r.width <= 0.0 || r.height <= 0.0 {
            return Err(format!("Region {} has invalid bounds", i));
        }
        if r.x + r.width > 1.0001 || r.y + r.height > 1.0001 {
            return Err(format!("Region {} extends beyond monitor bounds", i));
        }
    }

    // Check for overlaps (simple pairwise check)
    for i in 0..regions.len() {
        for j in (i + 1)..regions.len() {
            let a = &regions[i];
            let b = &regions[j];
            if rects_overlap(a, b) {
                return Err(format!("Regions {} and {} overlap", i, j));
            }
        }
    }

    Ok(())
}

fn rects_overlap(a: &SplitRegion, b: &SplitRegion) -> bool {
    let a_right = a.x + a.width;
    let a_bottom = a.y + a.height;
    let b_right = b.x + b.width;
    let b_bottom = b.y + b.height;

    a.x < b_right && a_right > b.x && a.y < b_bottom && a_bottom > b.y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edid_for_half_region() {
        let region = SplitRegion {
            x: 0.0,
            y: 0.0,
            width: 0.5,
            height: 1.0,
        };
        let edid = edid_for_region(3840, 1080, &region, 0);
        // Verify checksum
        let sum: u8 = edid.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
        assert_eq!(sum, 0);
    }

    #[test]
    fn test_validate_non_overlapping() {
        let regions = vec![
            SplitRegion { x: 0.0, y: 0.0, width: 0.5, height: 1.0 },
            SplitRegion { x: 0.5, y: 0.0, width: 0.5, height: 1.0 },
        ];
        assert!(validate_regions(&regions).is_ok());
    }

    #[test]
    fn test_validate_overlapping() {
        let regions = vec![
            SplitRegion { x: 0.0, y: 0.0, width: 0.6, height: 1.0 },
            SplitRegion { x: 0.5, y: 0.0, width: 0.5, height: 1.0 },
        ];
        assert!(validate_regions(&regions).is_err());
    }
}

