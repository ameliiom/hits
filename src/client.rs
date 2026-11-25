//! The client: what the server said each hit did, drawn above the mob after it is rendered, facing
//! the camera: the numbers rising and fading, the hearts for a few seconds after a hit (the HUD's
//! own heart sprites; a mob with more than ten hearts shows its health as a number).
use crate::look::{self, Number};
use mod_api::{ClientHost, RenderLivingPostEvent, Text};
#[allow(unused_imports)]
use mod_api::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

/// What is known of one mob: its health and most (tenths read as health), its last hit's time,
/// and its rising numbers. Times are stamped when first drawn (game ticks, from the draw).
#[derive(Default)]
struct Mob {
    health: f32,
    max: f32,
    last: Option<i64>,
    numbers: Vec<(f32, Option<i64>)>,
}

static MOBS: Mutex<Option<HashMap<u32, Mob>>> = Mutex::new(None);

fn with<R>(f: impl FnOnce(&mut HashMap<u32, Mob>) -> R) -> R {
    f(MOBS.lock().unwrap_or_else(|e| e.into_inner()).get_or_insert_with(HashMap::new))
}

/// The server told of a hit.
pub fn hit(entity: u32, amount: f32, health: f32, max: f32) {
    with(|m| {
        let mob = m.entry(entity).or_default();
        mob.health = health;
        mob.max = max;
        mob.last = None;
        mob.numbers.push((amount, None));
    });
}

/// Draws `s` centred, one character at a time from the left, so numbers and hearts keep their
/// order whatever the game's language ("-3" turned into "3-" in Arabic, and a direction mark showed
/// as a box: both seen in the game).
fn left_to_right(d: &mut dyn mod_api::BlockEntityDrawContext, glyphs: &[(char, u32)], y: f32) {
    let glyphs: Vec<(Text, f32, u32)> = glyphs
        .iter()
        .map(|(ch, colour)| {
            let t = Text::literal(ch.to_string());
            let w = d.text_width(&t).unwrap_or(0.0);
            (t, w, *colour)
        })
        .collect();
    let mut x = -glyphs.iter().map(|g| g.1).sum::<f32>() / 2.0;
    for (t, w, colour) in glyphs {
        let _ = d.text(&t, [x, y], colour, false, Some(0));
        x += w;
    }
}

/// Every character of `s` in one colour.
fn coloured(s: &str, colour: u32) -> Vec<(char, u32)> {
    s.chars().map(|c| (c, colour)).collect()
}

fn on(host: &dyn ClientHost, key: &str) -> bool {
    host.setting(crate::MOD_ID, key).is_none_or(|v| v.trim() != "false")
}

pub fn draw(host: &dyn ClientHost, event: &mut RenderLivingPostEvent<'_>) {
    let (numbers_on, hearts_on) = (on(host, crate::setting::NUMBERS), on(host, crate::setting::HEARTS));
    if !numbers_on && !hearts_on {
        return;
    }
    let id = event.entity.id;
    let d = &mut *event.draw;
    let now = d.game_time();
    let Some((health, max, numbers, show_hearts)) = with(|m| {
        let mob = m.get_mut(&id)?;
        let last = *mob.last.get_or_insert(now);
        for n in &mut mob.numbers {
            n.1.get_or_insert(now);
        }
        mob.numbers.retain(|(a, at)| look::rise(&Number { amount: *a, at: at.unwrap_or(now) }, now).is_some());
        let hearts = now - last < look::HEARTS_TICKS;
        if !hearts && mob.numbers.is_empty() {
            m.remove(&id);
            return None;
        }
        Some((mob.health, mob.max, mob.numbers.clone(), hearts))
    }) else {
        return;
    };
    let read: &dyn mod_api::Host = host;
    let height = read.entity(id).and_then(|e| e.dimensions(read)).map_or(2.0, |s| s[1] as f32);
    let camera = host.camera().map_or(0.0, |c| c.yaw);
    d.push();
    d.translate([0.0, height + 0.4, 0.0]);
    d.rotate_about([0.0, 1.0, 0.0], look::facing(event.entity.body_yaw, camera), [0.0, 0.0, 0.0]);
    d.scale([0.025, -0.025, 0.025]);
    if show_hearts && hearts_on {
        let row = look::heart_row(health, max, 10)
            .unwrap_or_else(|| coloured(&format!("❤ {} / {}", look::text(health), look::text(max)), 0xFF4040));
        left_to_right(d, &row, 0.0);
    }
    for (amount, at) in numbers.into_iter().filter(|_| numbers_on) {
        let Some(rise) = look::rise(&Number { amount, at: at.unwrap_or(now) }, now) else { continue };
        let (colour, scale) = look::colour_and_scale(amount, max);
        d.push();
        d.translate([0.0, -12.0 - rise, 0.0]);
        d.scale([scale, scale, 1.0]);
        left_to_right(d, &coloured(&look::label(amount), colour), -8.0);
        d.pop();
    }
    d.pop();
}
