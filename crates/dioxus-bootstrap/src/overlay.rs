/// Overlay placement relative to a trigger element.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum OverlayPlacement {
    /// Choose the first fitting fallback placement.
    Auto,
    /// Place overlay above the trigger.
    #[default]
    Top,
    /// Place overlay below the trigger.
    Bottom,
    /// Place overlay before the trigger in the inline axis.
    Start,
    /// Place overlay after the trigger in the inline axis.
    End,
}

impl OverlayPlacement {
    /// Default fallback order matching Bootstrap's Popper-backed overlays.
    pub const DEFAULT_FALLBACKS: [OverlayPlacement; 4] = [
        OverlayPlacement::Top,
        OverlayPlacement::End,
        OverlayPlacement::Bottom,
        OverlayPlacement::Start,
    ];
}

/// Rectangle in viewport coordinates.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OverlayRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl OverlayRect {
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn right(self) -> f64 {
        self.x + self.width
    }

    pub fn bottom(self) -> f64 {
        self.y + self.height
    }

    pub fn center_x(self) -> f64 {
        self.x + self.width / 2.0
    }

    pub fn center_y(self) -> f64 {
        self.y + self.height / 2.0
    }
}

/// Overlay offset from the trigger.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OverlayOffset {
    /// Cross-axis offset.
    pub skidding: f64,
    /// Main-axis distance from the trigger.
    pub distance: f64,
}

impl OverlayOffset {
    pub const ZERO: Self = Self {
        skidding: 0.0,
        distance: 0.0,
    };

    /// Bootstrap tooltip default offset.
    pub const TOOLTIP: Self = Self {
        skidding: 0.0,
        distance: 6.0,
    };

    /// Bootstrap popover default offset.
    pub const POPOVER: Self = Self {
        skidding: 0.0,
        distance: 8.0,
    };
}

/// Bootstrap arrow width (`--bs-popover-arrow-width` / tooltip equivalent, `1rem`).
const ARROW_SIZE: f64 = 16.0;
/// Keep the arrow this far from the overlay's rounded corner so it never straddles
/// the border radius.
const ARROW_EDGE_INSET: f64 = 8.0;

/// Calculated overlay position.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OverlayPosition {
    pub x: f64,
    pub y: f64,
    pub placement: OverlayPlacement,
    /// True when the overlay fits inside the boundary without clamping.
    pub fits: bool,
    /// Arrow centre in overlay-local coordinates along the cross axis (x for
    /// top/bottom placements, y for start/end). Lets the caller keep the arrow
    /// pointing at the trigger even after the overlay box is clamped to the
    /// viewport — the job Popper.js does for Bootstrap's own overlays.
    pub arrow: f64,
}

impl OverlayPosition {
    pub fn rect(self, overlay_size: OverlayRect) -> OverlayRect {
        OverlayRect::new(self.x, self.y, overlay_size.width, overlay_size.height)
    }
}

/// Arrow centre (cross-axis, overlay-local) that keeps the arrow over the trigger
/// centre, clamped so it stays clear of the overlay's rounded corners.
fn arrow_offset(trigger: OverlayRect, rect: OverlayRect, placement: OverlayPlacement) -> f64 {
    let (target, cross_size) = match placement {
        OverlayPlacement::Start | OverlayPlacement::End => {
            (trigger.center_y() - rect.y, rect.height)
        }
        // Top / Bottom / Auto place on the vertical axis, so the arrow slides in x.
        _ => (trigger.center_x() - rect.x, rect.width),
    };
    let lo = ARROW_EDGE_INSET + ARROW_SIZE / 2.0;
    let hi = (cross_size - ARROW_EDGE_INSET - ARROW_SIZE / 2.0).max(lo);
    target.clamp(lo, hi)
}

/// Calculate a viewport-aware overlay position.
///
/// `overlay_size.x` and `overlay_size.y` are ignored; only width and height are
/// used. If no candidate fully fits, the placement with the largest visible
/// area is selected and clamped inside the padded boundary as far as possible.
pub fn calculate_overlay_position(
    trigger: OverlayRect,
    overlay_size: OverlayRect,
    boundary: OverlayRect,
    requested: OverlayPlacement,
    fallback_placements: &[OverlayPlacement],
    offset: OverlayOffset,
    boundary_padding: f64,
) -> OverlayPosition {
    let candidates = candidate_placements(requested, fallback_placements);
    let mut best: Option<(OverlayPlacement, OverlayRect, f64)> = None;

    for placement in candidates {
        let rect = placed_rect(trigger, overlay_size, placement, offset);
        if fits_boundary(rect, boundary, boundary_padding) {
            return OverlayPosition {
                x: rect.x,
                y: rect.y,
                placement,
                fits: true,
                arrow: arrow_offset(trigger, rect, placement),
            };
        }

        let visible = visible_area(rect, boundary, boundary_padding);
        if best
            .map(|(_, _, best_visible)| visible > best_visible)
            .unwrap_or(true)
        {
            best = Some((placement, rect, visible));
        }
    }

    let (placement, rect, _) = best.unwrap_or_else(|| {
        let placement = OverlayPlacement::Top;
        (
            placement,
            placed_rect(trigger, overlay_size, placement, offset),
            0.0,
        )
    });
    let clamped = clamp_to_boundary(rect, boundary, boundary_padding);

    OverlayPosition {
        x: clamped.x,
        y: clamped.y,
        placement,
        fits: false,
        arrow: arrow_offset(trigger, clamped, placement),
    }
}

fn candidate_placements(
    requested: OverlayPlacement,
    fallback_placements: &[OverlayPlacement],
) -> Vec<OverlayPlacement> {
    let mut candidates = Vec::new();

    if requested == OverlayPlacement::Auto {
        push_candidates(&mut candidates, fallback_placements);
        if candidates.is_empty() {
            push_candidates(&mut candidates, &OverlayPlacement::DEFAULT_FALLBACKS);
        }
    } else {
        candidates.push(requested);
        push_candidates(&mut candidates, fallback_placements);
    }

    candidates
}

fn push_candidates(candidates: &mut Vec<OverlayPlacement>, placements: &[OverlayPlacement]) {
    for placement in placements {
        if *placement != OverlayPlacement::Auto && !candidates.contains(placement) {
            candidates.push(*placement);
        }
    }
}

fn placed_rect(
    trigger: OverlayRect,
    overlay_size: OverlayRect,
    placement: OverlayPlacement,
    offset: OverlayOffset,
) -> OverlayRect {
    match placement {
        OverlayPlacement::Auto => placed_rect(trigger, overlay_size, OverlayPlacement::Top, offset),
        OverlayPlacement::Top => OverlayRect::new(
            trigger.center_x() - overlay_size.width / 2.0 + offset.skidding,
            trigger.y - overlay_size.height - offset.distance,
            overlay_size.width,
            overlay_size.height,
        ),
        OverlayPlacement::Bottom => OverlayRect::new(
            trigger.center_x() - overlay_size.width / 2.0 + offset.skidding,
            trigger.bottom() + offset.distance,
            overlay_size.width,
            overlay_size.height,
        ),
        OverlayPlacement::Start => OverlayRect::new(
            trigger.x - overlay_size.width - offset.distance,
            trigger.center_y() - overlay_size.height / 2.0 + offset.skidding,
            overlay_size.width,
            overlay_size.height,
        ),
        OverlayPlacement::End => OverlayRect::new(
            trigger.right() + offset.distance,
            trigger.center_y() - overlay_size.height / 2.0 + offset.skidding,
            overlay_size.width,
            overlay_size.height,
        ),
    }
}

fn fits_boundary(rect: OverlayRect, boundary: OverlayRect, padding: f64) -> bool {
    rect.x >= boundary.x + padding
        && rect.y >= boundary.y + padding
        && rect.right() <= boundary.right() - padding
        && rect.bottom() <= boundary.bottom() - padding
}

fn visible_area(rect: OverlayRect, boundary: OverlayRect, padding: f64) -> f64 {
    let min_x = boundary.x + padding;
    let min_y = boundary.y + padding;
    let max_x = boundary.right() - padding;
    let max_y = boundary.bottom() - padding;

    let width = (rect.right().min(max_x) - rect.x.max(min_x)).max(0.0);
    let height = (rect.bottom().min(max_y) - rect.y.max(min_y)).max(0.0);
    width * height
}

fn clamp_to_boundary(rect: OverlayRect, boundary: OverlayRect, padding: f64) -> OverlayRect {
    let min_x = boundary.x + padding;
    let min_y = boundary.y + padding;
    let max_x = (boundary.right() - padding - rect.width).max(min_x);
    let max_y = (boundary.bottom() - padding - rect.height).max(min_y);

    OverlayRect::new(
        rect.x.clamp(min_x, max_x),
        rect.y.clamp(min_y, max_y),
        rect.width,
        rect.height,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trigger() -> OverlayRect {
        OverlayRect::new(100.0, 100.0, 40.0, 20.0)
    }

    fn overlay() -> OverlayRect {
        OverlayRect::new(0.0, 0.0, 80.0, 30.0)
    }

    fn boundary() -> OverlayRect {
        OverlayRect::new(0.0, 0.0, 300.0, 300.0)
    }

    #[test]
    fn requested_top_fits() {
        let position = calculate_overlay_position(
            trigger(),
            overlay(),
            boundary(),
            OverlayPlacement::Top,
            &OverlayPlacement::DEFAULT_FALLBACKS,
            OverlayOffset::TOOLTIP,
            0.0,
        );

        assert_eq!(position.placement, OverlayPlacement::Top);
        assert!(position.fits);
        assert_eq!(position.x, 80.0);
        assert_eq!(position.y, 64.0);
    }

    #[test]
    fn offset_skids_on_cross_axis() {
        let position = calculate_overlay_position(
            trigger(),
            overlay(),
            boundary(),
            OverlayPlacement::Bottom,
            &[],
            OverlayOffset {
                skidding: 10.0,
                distance: 12.0,
            },
            0.0,
        );

        assert_eq!(position.placement, OverlayPlacement::Bottom);
        assert!(position.fits);
        assert_eq!(position.x, 90.0);
        assert_eq!(position.y, 132.0);
    }

    #[test]
    fn falls_back_when_requested_placement_overflows() {
        let edge_trigger = OverlayRect::new(100.0, 10.0, 40.0, 20.0);
        let position = calculate_overlay_position(
            edge_trigger,
            overlay(),
            boundary(),
            OverlayPlacement::Top,
            &[OverlayPlacement::Bottom, OverlayPlacement::End],
            OverlayOffset::TOOLTIP,
            0.0,
        );

        assert_eq!(position.placement, OverlayPlacement::Bottom);
        assert!(position.fits);
        assert_eq!(position.y, 36.0);
    }

    #[test]
    fn auto_uses_first_fitting_fallback() {
        let edge_trigger = OverlayRect::new(100.0, 10.0, 40.0, 20.0);
        let position = calculate_overlay_position(
            edge_trigger,
            overlay(),
            boundary(),
            OverlayPlacement::Auto,
            &[
                OverlayPlacement::Top,
                OverlayPlacement::Bottom,
                OverlayPlacement::End,
            ],
            OverlayOffset::TOOLTIP,
            0.0,
        );

        assert_eq!(position.placement, OverlayPlacement::Bottom);
        assert!(position.fits);
    }

    #[test]
    fn start_and_end_place_on_inline_axis() {
        let start = calculate_overlay_position(
            trigger(),
            overlay(),
            boundary(),
            OverlayPlacement::Start,
            &[],
            OverlayOffset::POPOVER,
            0.0,
        );
        let end = calculate_overlay_position(
            trigger(),
            overlay(),
            boundary(),
            OverlayPlacement::End,
            &[],
            OverlayOffset::POPOVER,
            0.0,
        );

        assert_eq!(start.x, 12.0);
        assert_eq!(start.y, 95.0);
        assert_eq!(end.x, 148.0);
        assert_eq!(end.y, 95.0);
    }

    #[test]
    fn clamps_best_candidate_to_boundary_padding() {
        let edge_trigger = OverlayRect::new(0.0, 120.0, 20.0, 20.0);
        let position = calculate_overlay_position(
            edge_trigger,
            overlay(),
            boundary(),
            OverlayPlacement::Top,
            &[],
            OverlayOffset::ZERO,
            8.0,
        );

        assert_eq!(position.placement, OverlayPlacement::Top);
        assert!(!position.fits);
        assert_eq!(position.x, 8.0);
        assert_eq!(position.y, 90.0);
    }

    #[test]
    fn arrow_is_centred_when_overlay_fits() {
        // Centred trigger, overlay fits: the arrow sits at the overlay's centre.
        let position = calculate_overlay_position(
            OverlayRect::new(130.0, 20.0, 40.0, 20.0), // center_x = 150
            OverlayRect::new(0.0, 0.0, 80.0, 30.0),
            boundary(),
            OverlayPlacement::Bottom,
            &[],
            OverlayOffset::ZERO,
            0.0,
        );
        assert!(position.fits);
        assert_eq!(position.x, 110.0);
        // 150 - 110 = 40 = overlay width / 2 (centred).
        assert_eq!(position.arrow, 40.0);
    }

    #[test]
    fn arrow_tracks_trigger_after_horizontal_clamp() {
        // A trigger near the right edge: the bottom overlay is centred on it,
        // overflows the boundary, and is clamped left. The arrow must keep pointing
        // at the trigger centre, not drift to the (now shifted) overlay centre.
        let position = calculate_overlay_position(
            OverlayRect::new(280.0, 20.0, 20.0, 20.0), // center_x = 290
            OverlayRect::new(0.0, 0.0, 120.0, 60.0),
            boundary(), // 300 wide
            OverlayPlacement::Bottom,
            &[],
            OverlayOffset::ZERO,
            0.0,
        );
        assert_eq!(position.placement, OverlayPlacement::Bottom);
        assert!(!position.fits);
        assert_eq!(position.x, 180.0); // clamped so right edge hits 300
        // trigger.center_x - x = 290 - 180 = 110, clamped to [16, 104] -> 104:
        // the arrow hugs the trigger side, not the overlay centre (60).
        assert_eq!(position.arrow, 104.0);
    }

    #[test]
    fn clamps_oversized_overlay_to_boundary_start() {
        let oversized = OverlayRect::new(0.0, 0.0, 400.0, 400.0);
        let position = calculate_overlay_position(
            trigger(),
            oversized,
            boundary(),
            OverlayPlacement::Bottom,
            &[],
            OverlayOffset::ZERO,
            8.0,
        );

        assert!(!position.fits);
        assert_eq!(position.x, 8.0);
        assert_eq!(position.y, 8.0);
    }
}
