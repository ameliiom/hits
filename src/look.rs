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

/// Hearts: how many full and whether a half, for `health` (2 health a heart), at most `cap` hearts
/// drawn (bigger mobs show a number instead).
pub fn hearts(health: f32, cap: u32) -> Option<(u32, bool)> {
    let halves = (health.max(0.0)).ceil() as u32;
    let (full, half) = (halves / 2, halves % 2 == 1);
    (full + u32::from(half) <= cap).then_some((full, half))
}

#[cfg(test)]
mod tests {
    use super::*;


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
    fn hearts_round_up_to_halves() {
        assert_eq!(hearts(20.0, 10), Some((10, false)));
        assert_eq!(hearts(7.0, 10), Some((3, true)));
        assert_eq!(hearts(0.5, 10), Some((0, true)));
        assert_eq!(hearts(40.0, 10), None);
    }
}
