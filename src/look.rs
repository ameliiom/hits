//! How a hit looks, on plain values: the number's text, colour, size and rise, how long the hearts
//! stay, and the turn that faces text to the camera.

/// How long a number rises and the hearts stay, in game ticks.
pub const NUMBER_TICKS: i64 = 20;
pub const HEARTS_TICKS: i64 = 100;
/// How far a number rises over its life, in font pixels.
pub const RISE: f32 = 16.0;

/// A hit as the client keeps it: how much, and when (game ticks).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Number {
    pub amount: f32,
    pub at: i64,
}

/// The number's text: whole hits whole, the rest to one decimal ("3", "2.5").
pub fn text(amount: f32) -> String {
    let tenths = (amount * 10.0).round() as i64;
    if tenths % 10 == 0 { format!("{}", tenths / 10) } else { format!("{}.{}", tenths / 10, tenths % 10) }
}

/// Healing shows at least this much at once (a potion, a golden apple), so the slow regeneration
/// of players doesn't rain numbers.
pub const HEAL_SHOWN: f32 = 2.0;

/// A heal (a negative change) is green; a big hit (a quarter of the mob's life or more) bigger and
/// red; others yellow.
pub fn colour_and_scale(amount: f32, max: f32) -> (u32, f32) {
    if amount < 0.0 {
        (0x55FF55, 1.0)
    } else if max > 0.0 && amount >= max / 4.0 {
        (0xFF5555, 1.5)
    } else {
        (0xFFDD33, 1.0)
    }
}

/// How much a heal really gave: what health rose by since it was last known, at most what was
/// asked (an Instant Health III asks 32 of a pig that can take 3); unknown before, at most the
/// most it can hold.
pub fn healed(asked: f32, before: Option<f32>, after: f32, max: f32) -> f32 {
    match before {
        Some(b) => (after - b).clamp(0.0, asked),
        None => asked.min(max),
    }
}

/// The number shown: "-3" for a hit, "+4" for a heal.
pub fn label(amount: f32) -> String {
    if amount < 0.0 { format!("+{}", text(-amount)) } else { format!("-{}", text(amount)) }
}

/// How far a number has risen at `now`, or None once it is gone.
pub fn rise(n: &Number, now: i64) -> Option<f32> {
    let age = now - n.at;
    (0..NUMBER_TICKS).contains(&age).then(|| RISE * age as f32 / NUMBER_TICKS as f32)
}

/// The turn about the vertical axis that faces text drawn after a mob's body to the camera: the
/// draw comes turned with the body, so the body's turn is undone and the camera's applied.
pub fn facing(body_yaw: f32, camera_yaw: f32) -> f32 {
    (180.0 + body_yaw - camera_yaw).rem_euclid(360.0)
}

/// The hearts row: each heart's glyph and colour, full red, a half pale, the empty ones dark (the
/// font's outline heart drew as red as a full one: seen in the game); None for a mob with more
/// than `cap` hearts.
pub fn heart_row(health: f32, max: f32, cap: u32) -> Option<Vec<(char, u32)>> {
    let total = (max / 2.0).ceil().max(0.0) as u32;
    if total > cap {
        return None;
    }
    let (full, half) = hearts(health, cap)?;
    Some(
        (0..total)
            .map(|i| match i.cmp(&full) {
                std::cmp::Ordering::Less => ('❤', 0xFF4040),
                std::cmp::Ordering::Equal if half => ('❤', 0xFF9A9A),
                _ => ('❤', 0x3A3A3A),
            })
            .collect(),
    )
}

/// Hearts: how many full and whether a half, for `health` (2 health a heart), at most `cap` hearts
/// drawn (bigger mobs show a number instead).
pub fn hearts(health: f32, cap: u32) -> Option<(u32, bool)> {
    let halves = (health.max(0.0)).ceil() as u32;
    let (full, half) = (halves / 2, halves % 2 == 1);
    (full + u32::from(half) <= cap).then_some((full, half))
}

/// Trades show for a merchant looked at within this many blocks, at most `TRADE_ROWS` of them.
pub const TRADE_RANGE: f64 = 8.0;
pub const TRADE_ROWS: usize = 6;

/// The first `TRADE_ROWS` of a merchant's offers, in its own order (the trading screen's).
pub fn capped<T>(offers: impl IntoIterator<Item = T>) -> Vec<T> {
    offers.into_iter().take(TRADE_ROWS).collect()
}

/// Where a trade row's parts sit, in blocks from the row's centre: the first cost, the second
/// (when there is one), the arrow and the result, left to right.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RowLayout {
    pub cost_a: f32,
    pub cost_b: Option<f32>,
    pub arrow: f32,
    pub result: f32,
}

/// One icon's slot and one row's height, in blocks.
pub const ICON: f32 = 0.26;
pub const ROW_HEIGHT: f32 = 0.28;

/// A row laid out centred, a slot `ICON` wide for each part (the arrow too).
pub fn row_layout(two_costs: bool) -> RowLayout {
    let slots = if two_costs { 4.0 } else { 3.0 };
    let x = |i: f32| (i - (slots - 1.0) / 2.0) * ICON;
    if two_costs {
        RowLayout { cost_a: x(0.0), cost_b: Some(x(1.0)), arrow: x(2.0), result: x(3.0) }
    } else {
        RowLayout { cost_a: x(0.0), cost_b: None, arrow: x(1.0), result: x(2.0) }
    }
}

/// Trades sit in this many columns, so 6 make 3 rows and the stack stays low enough to clear the
/// Jade panel from up close (6 rows in one column went under it at 4 blocks: seen in the game).
pub const COLUMNS: usize = 2;
/// From one column's centre to the next: the widest row (4 slots) and half a slot between.
pub const COLUMN_WIDTH: f32 = 4.5 * ICON;

/// Where trade `i` of `n` is centred, in blocks: x from the middle, y above the lowest row. Read
/// left to right, then down, the first trade at the top left; one trade is centred.
pub fn place(i: usize, n: usize) -> [f32; 2] {
    let columns = n.clamp(1, COLUMNS);
    let rows = n.div_ceil(columns);
    let (row, column) = (i / columns, i % columns);
    let x = (column as f32 - (columns - 1) as f32 / 2.0) * COLUMN_WIDTH;
    [x, rows.saturating_sub(1).saturating_sub(row) as f32 * ROW_HEIGHT]
}

/// How far along the ray from `origin` (unit `dir`) it first meets the box `min`..`max`, or None
/// for a miss or a box behind it (the slab test; from inside the box, 0).
pub fn ray_box(origin: [f64; 3], dir: [f64; 3], min: [f64; 3], max: [f64; 3]) -> Option<f64> {
    let (mut near, mut far) = (0.0_f64, f64::INFINITY);
    for i in 0..3 {
        if dir[i].abs() < 1e-12 {
            if origin[i] < min[i] || origin[i] > max[i] {
                return None;
            }
            continue;
        }
        let (a, b) = ((min[i] - origin[i]) / dir[i], (max[i] - origin[i]) / dir[i]);
        near = near.max(a.min(b));
        far = far.min(a.max(b));
        if near > far {
            return None;
        }
    }
    Some(near)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trades_are_capped_in_their_order() {
        assert_eq!(capped(1..=10), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(capped(vec![7, 8]), vec![7, 8]);
        assert!(capped(Vec::<u8>::new()).is_empty());
    }

    #[test]
    fn a_row_is_centred_and_the_first_trade_is_on_top() {
        let one = row_layout(false);
        assert_eq!((one.cost_a, one.cost_b, one.arrow, one.result), (-ICON, None, 0.0, ICON));
        let two = row_layout(true);
        assert_eq!(two.cost_b, Some(-ICON / 2.0));
        assert!((two.cost_a + two.result).abs() < 1e-6 && two.arrow > 0.0 && two.result > two.arrow);
        // Six trades: 3 rows of 2, the first top left, the last bottom right.
        assert_eq!(place(0, 6), [-COLUMN_WIDTH / 2.0, 2.0 * ROW_HEIGHT]);
        assert_eq!(place(1, 6), [COLUMN_WIDTH / 2.0, 2.0 * ROW_HEIGHT]);
        assert_eq!(place(5, 6), [COLUMN_WIDTH / 2.0, 0.0]);
        // Odd counts keep their columns; one trade is centred.
        assert_eq!(place(2, 3), [-COLUMN_WIDTH / 2.0, 0.0]);
        assert_eq!(place(0, 1), [0.0, 0.0]);
        // Two neighbouring four-slot rows never overlap.
        assert!(COLUMN_WIDTH >= 4.0 * ICON);
    }

    #[test]
    fn the_look_ray_meets_boxes_ahead_only() {
        let b = ([4.0, -1.0, -0.5], [5.0, 1.0, 0.5]);
        assert_eq!(ray_box([0.0; 3], [1.0, 0.0, 0.0], b.0, b.1), Some(4.0));
        assert_eq!(ray_box([0.0; 3], [-1.0, 0.0, 0.0], b.0, b.1), None);
        assert_eq!(ray_box([0.0; 3], [0.0, 1.0, 0.0], b.0, b.1), None);
        assert_eq!(ray_box([4.5, 0.0, 0.0], [0.0, 0.0, 1.0], b.0, b.1), Some(0.0));
    }


    #[test]
    fn numbers_read_cleanly() {
        assert_eq!(text(3.0), "3");
        assert_eq!(text(2.5), "2.5");
        assert_eq!(text(0.04), "0");
        assert_eq!(text(12.25), "12.3");
    }

    #[test]
    fn big_hits_stand_out_and_heals_are_green() {
        assert_eq!(colour_and_scale(5.0, 20.0), (0xFF5555, 1.5));
        assert_eq!(colour_and_scale(4.0, 20.0), (0xFFDD33, 1.0));
        assert_eq!(colour_and_scale(-4.0, 20.0), (0x55FF55, 1.0));
        assert_eq!((label(3.0), label(-4.0), label(2.5)), ("-3".into(), "+4".into(), "-2.5".into()));
    }

    #[test]
    fn a_heal_shows_what_it_really_gave() {
        // Seen in the game: Instant Health III on a pig at 6.5 of 10 showed "+32".
        assert_eq!(healed(32.0, Some(6.5), 10.0, 10.0), 3.5);
        assert_eq!(healed(4.0, Some(2.0), 6.0, 20.0), 4.0);
        assert_eq!(healed(32.0, None, 10.0, 10.0), 10.0);
        assert_eq!(healed(4.0, Some(9.0), 8.0, 10.0), 0.0);
    }

    #[test]
    fn a_number_rises_then_goes() {
        let n = Number { amount: 3.0, at: 100 };
        assert_eq!(rise(&n, 100), Some(0.0));
        assert_eq!(rise(&n, 110), Some(RISE / 2.0));
        assert_eq!(rise(&n, 120), None);
        assert_eq!(rise(&n, 99), None);
    }

    #[test]
    fn facing_matches_the_two_measured_views() {
        // Measured: a cow turned 90°, the camera at 180°, faced with a turn of 90°; at camera 90°
        // the turns 0° failed, so 180° is the one left.
        assert_eq!(facing(90.0, 180.0), 90.0);
        assert_eq!(facing(90.0, 90.0), 180.0);
        assert_eq!(facing(0.0, 0.0), 180.0);
    }

    #[test]
    fn the_hearts_row_shows_empty_hearts_dark() {
        let row = heart_row(5.0, 10.0, 10).unwrap();
        assert_eq!(row.iter().map(|h| h.1).collect::<Vec<_>>(), vec![0xFF4040, 0xFF4040, 0xFF9A9A, 0x3A3A3A, 0x3A3A3A]);
        assert!(heart_row(30.0, 40.0, 10).is_none());
    }

    #[test]
    fn hearts_round_up_to_halves() {
        assert_eq!(hearts(20.0, 10), Some((10, false)));
        assert_eq!(hearts(7.0, 10), Some((3, true)));
        assert_eq!(hearts(0.5, 10), Some((0, true)));
        assert_eq!(hearts(40.0, 10), None);
    }
}
