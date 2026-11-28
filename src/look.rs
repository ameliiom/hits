//! How a hit looks, on plain values: the number's text, colour, size and rise, and the turn that
//! faces text to the camera.

/// How long a number rises, in game ticks.
pub const NUMBER_TICKS: i64 = 20;
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
}
